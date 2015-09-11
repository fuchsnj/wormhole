use hyper;
use std::net::ToSocketAddrs;
use status_code::StatusCode;
use request;
use handler::{Action, Handler};
use body::Body;
use std::io::{Read, Write};
use std::io;
use std::marker::PhantomData;
use hyper::server::request::Request as HyperRequest;
use hyper::server::response::Response as HyperResponse;
use hyper::net::Fresh as HyperFresh;

struct NullWriter;
impl Write for NullWriter{
	fn write(&mut self, buf: &[u8]) -> io::Result<usize>{
		Ok(buf.len())
	}
	fn flush(&mut self) -> io::Result<()>{Ok(())}
}

struct HyperHandler<H, D2, E>{
	handler: H,
	phantom: PhantomData<fn(D2) -> E>,
	cookie_key: Vec<u8>
}
impl<H, D2, E> hyper::server::Handler for HyperHandler<H, D2, E>
where H: Handler<(), D2, E> + 'static{
	fn handle<'a, 'k>(&'a self, req: HyperRequest<'a, 'k>, mut res: HyperResponse<'a, HyperFresh>){
		*res.status_mut() = StatusCode::InternalServerError;//error returned if thread panics
		let mut request = request::new(req, res, &self.cookie_key);
		let (status_code, body) = match self.handler.handle(&mut request, () ){
		
			//TODO: should D2 be generic, or always '()'?
			Ok(Action::Next(_)) => (StatusCode::NotFound, Box::new("404 - Not Found") as Box<Body>),
			Ok(Action::Done(data)) => data,
			Err(err) => (StatusCode::InternalServerError, Box::new("500 - Internal Server Error") as Box<Body>)
		};
		//let body2: Box<Body> = body;
		let (mut req, mut res) = request::deconstruct(request);
		*res.status_mut() = status_code;
		match res.start(){
			Ok(mut stream) => {
				body.write_to(&mut stream);
			},
			Err(_) => {
				println!("failed to obtain HTTP output stream!");
			}
		};
		//make sure all bytes from the Request are read
		//fixes bug #309 in Hyper
		let _ = io::copy(&mut req, &mut NullWriter);
	}
}

impl<H, D2, E> HyperHandler<H, D2, E>
where H: Handler<(), D2, E> + 'static{
	fn new(handler: H, cookie_key: &[u8]) -> HyperHandler<H, D2, E>{
		HyperHandler{
			handler: handler,
			phantom: PhantomData,
			cookie_key: Vec::from(cookie_key)
		}
	}
}


pub struct Server{
	running: Option<hyper::server::Listening>,
	cookie_key: Vec<u8>
}
impl Server{
	pub fn http() -> Server{
		Server{
			running: None,
			cookie_key: vec!(0)
		}
	}
	
	pub fn set_cookie_key(&mut self, key: &[u8]){
		self.cookie_key = Vec::from(key);
	}
	
	pub fn start<A, H, D2, E>(&mut self, addr: A, handler: H) where
	A: ToSocketAddrs,
	H: Handler<(), D2, E> + 'static,
	E: 'static,
	D2: 'static{
		let server = hyper::Server::http(addr).unwrap();
		let cookie_key = self.cookie_key.clone();
		let handler = HyperHandler::new(handler, &self.cookie_key);
		let listening = server.handle(handler).unwrap();
		/*
		let listening = server.handle(move |req: hyper::server::request::Request, mut res: hyper::server::response::Response<hyper::net::Fresh>|{
			let ck = cookie_key;
			
		}).unwrap();
		*/
		self.running = Some(listening);
	}
	
	pub fn stop(&mut self){
		self.running = None;
	}
}

