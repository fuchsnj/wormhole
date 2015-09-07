use server::Server;
use body::Body;
use status_code::StatusCode;
use handler::{Action, NormalHandler};
use request::Request;
use {request, server, hyper};
use std::net::ToSocketAddrs;

pub trait StartServer<D2, E>{
	fn start<A>(self, addr: A) -> Server
	where A: ToSocketAddrs;
}

impl<D2, E, T> StartServer<D2, E> for T where T: NormalHandler<(), D2, E> + 'static{
	fn start<A>(self, addr: A) -> Server
	where A: ToSocketAddrs{
		let server = hyper::Server::http(addr).unwrap();
		let listening = server.handle(move |req: hyper::server::request::Request, mut res: hyper::server::response::Response<hyper::net::Fresh>|{
			*res.status_mut() = StatusCode::InternalServerError;//error returned if thread panics
			let mut request = request::new(req, res);
			let (status_code, body) = match self.handle(&mut request, () ){
				Ok(Action::Next(_)) => (StatusCode::NotFound, Box::new("404 - Not Found") as Box<Body>),
				Ok(Action::Done(data)) => data,
				Err(_) => (StatusCode::InternalServerError, Box::new("500 - Internal Server Error") as Box<Body>)
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