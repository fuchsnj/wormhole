#![feature(default_type_parameter_fallback)]
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
use handler::{HandlerResult, Handler, ErrorHandler, ParamHandler, PathHandler, Action};
use request::Request;

use server::Server;
use hyper;
use status_code::StatusCode;
use request;
use server;
use body::Body;
use std::marker::PhantomData;
use std::sync::Arc;
use method::Method;
use header;
use unicase::UniCase;


pub struct Route<D1, D2, E>{
	handler: Arc<Handler<D1, D2, E> + Send + Sync + 'static>
}


pub fn route<D1,E>() -> Route<D1, D1, E>{
	Route{
		handler: Arc::new(())
	}
}

impl<D1: 'static, D2: Clone + 'static, E: 'static> Route<D1, D2, E>{
	pub fn using<H2: 'static, D3>(self, handler: H2) -> Route<D1, D3, E>
	where H2: Send + Sync + Handler<D2, D3, E>{
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
	
	pub fn param<H2, H3, D3, D4>(self, param_handler: H2, handler: H3) -> Route<D1, D2, E> where
	H2: Send + Sync + ParamHandler<D2, D3, E> + 'static,
	H3: Send + Sync + Handler<D3, D4, E> + 'static{
		self.path(move |req: &mut Request, data: D2, path: Option<&str>|{
			match path{
				Some(param) => {
					let data3: D3 = match param_handler.handle_param(req, data.clone(), param){
						Ok(Action::Next(data)) => data,
						Ok(Action::Done(res)) => return Ok(Action::Done(res)),
						Err(err) => return Err(err)
					};
					match handler.handle(req, data3){
						Ok(Action::Next(_)) => Ok(Action::Next(data)),
						Ok(Action::Done(res)) => Ok(Action::Done(res)),
						Err(err) => Err(err)
					}
				},
				None => req.next(data)
			}
		})
	}
	
	pub fn route<H2, D3>(self, path: &str, handler: H2) -> Route<D1, D2, E>
	where H2: Send + Sync + Handler<D2, D3, E> + 'static{
		let path = path.to_owned();
		self.path(move |req: &mut Request, data: D2, p: Option<&str>|{
			match p{
				Some(param) => {
					match param == path{
						true => match handler.handle(req, data.clone()){
							Ok(Action::Next(_)) => Ok(Action::Next(data)),
							Ok(Action::Done(res)) => Ok(Action::Done(res)),
							Err(err) => Err(err)
						},
						false => req.next(data)
					}
				},
				None => req.next(data)
			}
		})
	}
	
	pub fn root<H2>(self, handler: H2) -> Route<D1, D2, E>
	where H2: Send + Sync + Handler<D2, D2, E> + 'static{
		self.path(move |req: &mut Request, data, path: Option<&str>|{
			match path{
				Some(_) => req.next(data),
				None => handler.handle(req, data)
			}
		})
	}
	
	pub fn path<H2>(self, handler: H2) -> Route<D1, D2, E>
	where H2: Send + Sync + PathHandler<D2, E> + 'static{
		Route{
			handler: Arc::new(move |req: &mut Request, data|{
				let data:D2 = match self.handler.handle(req, data){
					Ok(Action::Next(data)) => data,
					Ok(Action::Done(res)) => return Ok(Action::Done(res)),
					Err(err) => return Err(err)
				};
				let path:String = req.get_path().to_owned();
				let (segment, path_remainder) = get_next_url_segment(&path);
				match segment{
					Some(segment) => {
						req.set_path(path_remainder);
						let result = handler.handle_path(req, data, Some(segment));
						req.set_path(&path);
						result
					},
					None => handler.handle_path(req, data, None)
				}
			})
		}
	}
	pub fn method<H2>(self, method: Method, handler: H2) -> Route<D1, D2, E>
	where H2: Send + Sync + Handler<D2, D2, E> + 'static{
		self.root(move |req: &mut Request, data|{
			match req.get_method() == &method{
				true => handler.handle(req, data),
				false => req.next(data)
			}
		})
	}
	pub fn get<H2>(self, handler: H2) -> Route<D1, D2, E>
	where H2: Handler<D2, D2, E> + 'static{
		self.method(Method::Get, handler)
	}
	pub fn post<H2>(self, handler: H2) -> Route<D1, D2, E>
	where H2: Handler<D2, D2, E> + 'static{
		self.method(Method::Post, handler)
	}
	pub fn put<H2>(self, handler: H2) -> Route<D1, D2, E>
	where H2: Handler<D2, D2, E> + 'static{
		self.method(Method::Put, handler)
	}
	pub fn delete<H2>(self, handler: H2) -> Route<D1, D2, E>
	where H2: Handler<D2, D2, E> + 'static{
		self.method(Method::Delete, handler)
	}
	pub fn head<H2>(self, handler: H2) -> Route<D1, D2, E>
	where H2: Handler<D2, D2, E> + 'static{
		self.method(Method::Head, handler)
	}
	pub fn trace<H2>(self, handler: H2) -> Route<D1, D2, E>
	where H2: Handler<D2, D2, E> + 'static{
		self.method(Method::Trace, handler)
	}
	pub fn connect<H2>(self, handler: H2) -> Route<D1, D2, E>
	where H2: Handler<D2, D2, E> + 'static{
		self.method(Method::Connect, handler)
	}
	pub fn patch<H2>(self, handler: H2) -> Route<D1, D2, E>
	where H2: Handler<D2, D2, E> + 'static{
		self.method(Method::Patch, handler)
	}
	pub fn options<H2>(self, handler: H2) -> Route<D1, D2, E>
	where H2: Handler<D2, D2, E> + 'static{
		self.method(Method::Options, handler)
	}
	pub fn cors(self) -> Route<D1, D2, E>{
		self.using(|req: &mut Request, data: D2| -> HandlerResult<D2, E>{
			if req.get_method() == &Method::Options {
				req.set_response_header(header::AccessControlAllowOrigin::Any);
				if let Some(requested_headers) = req.get_request_header::<header::AccessControlRequestHeaders>()
				.map(|h|h.0.clone()){
					req.set_response_header(header::AccessControlAllowHeaders(requested_headers));
				}
				//if let Some(ref headers) = req.get_request_header::<header::AccessControlRequestHeaders>(){
				//	req.set_response_header(header::AccessControlAllowHeaders(headers.0.clone()));
				//} 
				//req.get_request_header(
				//req.set_response_header(header::AccessControlAllowHeaders(vec!(
				//	UniCase("Content-Type".to_owned())
				//)));
				req.send(StatusCode::Ok, "")
			}else{
				req.set_response_header(header::AccessControlAllowOrigin::Any);
				req.next(data)
			}
		})
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
			b'?' => match a == segment_start{
				true => return ( None, &path[a..] ),
				false => return ( Some(&path[segment_start..a]), &path[a..] )
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
	handler1: Arc<Handler<D1, D2, E> + Send + Sync + 'static>,
	handler2: H2
}

impl<H2, D1, D2, D3, E> Handler<D1, D3, E> for ConcatHandler<H2, D1, D2, E> where
H2: Send + Sync + Handler<D2, D3, E>{
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
	handler1: Arc<Handler<D1, D2, E> + Send + Sync + 'static>,
	handler2: H2
}

impl<H2, D1, D2, E> Handler<D1, D2, E> for ConcatErrorHandler<H2, D1, D2, E> where
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

impl<D1, D2, E> Handler<D1, D2, E> for Route<D1, D2, E>{
	fn handle(&self, req: &mut Request, data: D1) -> HandlerResult<D2, E>{
		self.handler.handle(req, data)
	}
}