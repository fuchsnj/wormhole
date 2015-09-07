/*
mod route;
mod sub_route;
mod after_route;

mod after_middleware;
mod before_middleware;
mod root_middleware;
mod sub_middleware;
mod start_server;

pub use self::route::Route;
pub use self::sub_route::SubRoute;
pub use self::after_route::AfterRoute;

pub use self::after_middleware::{AddAfterMiddleware, AfterMiddlewareMethods};
pub use self::before_middleware::{AddBeforeMiddleware, BeforeMiddlewareMethods};
pub use self::root_middleware::{AddRootMiddleware, RootMiddlewareMethods};
pub use self::sub_middleware::{AddSubMiddleware, SubMiddlewareMethods};
pub use self::start_server::StartServer;
*/
use handler::{HandlerResult, NormalHandler, ErrorHandler, ParamHandler, Action};
use request::Request;
use std::net::ToSocketAddrs;
use server::Server;
use hyper;
use status_code::StatusCode;
use request;
use server;
use body::Body;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct Route<D1, D2, E>{
	handler: Arc<NormalHandler<D1, D2, E> + Send + Sync + 'static>
}


impl<D2: 'static, E: 'static> Route<(), D2, E>{
	pub fn start<A>(self, addr: A) -> Server
	where A: ToSocketAddrs{
		let server = hyper::Server::http(addr).unwrap();
		let listening = server.handle(move |req: hyper::server::request::Request, mut res: hyper::server::response::Response<hyper::net::Fresh>|{
			*res.status_mut() = StatusCode::InternalServerError;//error returned if thread panics
			let mut request = request::new(req, res);
			let (status_code, body) = match self.handler.handle(&mut request, () ){
			
				//TODO: should D2 be generic, or always '()'?
				Ok(Action::Next(_)) => (StatusCode::NotFound, Box::new("404 - Not Found") as Box<Body>),
				Ok(Action::Done(data)) => data,
				Err(err) => (StatusCode::InternalServerError, Box::new("500 - Internal Server Error") as Box<Body>)
			};
			//let body2: Box<Body> = body;
			let (req, mut res) = request::deconstruct(request);
			*res.status_mut() = status_code;
			match res.start(){
				Ok(mut stream) => {
					body.write_to(&mut stream);
				},
				Err(_) => {
					println!("failed to obtain HTTP output stream!");
				}
			};
		}).unwrap();
		server::new(listening)
	}
}

/*
impl<E> Route<(), (), E>{
	pub fn new() -> Route<(), (), (), E>{
		Route{
			handler: (),
			phantom: PhantomData
		}		
	}
}*/

pub fn route<E>() -> Route<(), (), E>{
	Route{
		handler: Arc::new(())
	}
}

impl<D1: 'static, D2: 'static, E: 'static> Route<D1, D2, E>{
	pub fn using<H2: 'static, D3>(self, handler: H2) -> Route<D1, D3, E>
	where H2: Send + Sync + NormalHandler<D2, D3, E>{
		Route{
			handler: Arc::new(ConcatHandler{
				handler1: self.handler,
				handler2: handler
			})
		}
	}
	pub fn catch<H2: 'static>(self, handler: H2) -> Route<D1, D2, E>
	where H2: Send + Sync + ErrorHandler<D2, E>{
		Route{
			handler: Arc::new(ConcatErrorHandler{
				handler1: self.handler,
				handler2: handler
			})
		}
	}
	//pub fn method<T>(mut self, method: Method, handler: T) -> SubRoute<D1, D2, E>
	//where T: NormalHandler<D1, D2, E> + 'static;
	
	pub fn param<H2, D3>(self, handler: H2) -> Route<D1, D3, E>
	where H2: Send + Sync + ParamHandler<D2, D3, E> + 'static{
		Route{
			handler: Arc::new(move |req: &mut Request, data| -> HandlerResult<D3, E>{
				let data:D2 = match self.handler.handle(req, data){
					Ok(Action::Next(data)) => data,
					Ok(Action::Done(res)) => return Ok(Action::Done(res)),
					Err(err) => return Err(err)
				};
				
				let path:String = req.get_path().to_owned();
				let (segment, path_remainder) = get_next_url_segment(&path);
				req.set_path(path_remainder);
				let result = handler.handle_param(req, data, segment);
				req.set_path(&path);
				result
			})
		}
	}
}

fn get_next_url_segment(mut path: &str) -> (Option<&str>, &str){
	let mut segment_start = 0;
	for a in 0..path.len(){
		match path.as_bytes()[a]{
			b'/' | b'\\' => {
				match a{
					0 => segment_start+=1,
					_ => return ( Some(&path[segment_start..a]), &path[a..] )
				}
			},
			b'?' => match a{
				0 => return ( None, &path[a..] ),
				_ => return ( Some(&path[segment_start..a]), &path[a..] )
			},
			_ => {}
		}
	}
	match path.len() > segment_start{
		true => ( Some(&path[segment_start..]), "" ),
		false => ( None, "" )
	}
}

pub struct ConcatHandler<H2, D1, D2, E>{
	handler1: Arc<NormalHandler<D1, D2, E> + Send + Sync + 'static>,
	handler2: H2
}

impl<H2, D1, D2, D3, E> NormalHandler<D1, D3, E> for ConcatHandler<H2, D1, D2, E> where
H2: Send + Sync + NormalHandler<D2, D3, E>{
	fn handle(&self, req: &mut Request, data: D1) -> HandlerResult<D3, E>{
		let data:D2 = match self.handler1.handle(req, data){
			Ok(Action::Next(data)) => data,
			Ok(Action::Done(res)) => return Ok(Action::Done(res)),
			Err(err) => return Err(err)
		};
		self.handler2.handle(req, data)
	}
}

pub struct ConcatErrorHandler<H2, D1, D2, E>{
	handler1: Arc<NormalHandler<D1, D2, E> + Send + Sync + 'static>,
	handler2: H2
}

impl<H2, D1, D2, E> NormalHandler<D1, D2, E> for ConcatErrorHandler<H2, D1, D2, E> where
H2: Send + Sync + ErrorHandler<D2, E>{
	fn handle(&self, req: &mut Request, data: D1) -> HandlerResult<D2, E>{
		let err:E = match self.handler1.handle(req, data){
			Ok(Action::Next(data)) => return Ok(Action::Next(data)),
			Ok(Action::Done(res)) => return Ok(Action::Done(res)),
			Err(err) => err
		};
		self.handler2.handle_error(req, err)
	}
}

impl<D1, D2, E> NormalHandler<D1, D2, E> for Route<D1, D2, E>{
	fn handle(&self, req: &mut Request, data: D1) -> HandlerResult<D2, E>{
		self.handler.handle(req, data)
	}
}