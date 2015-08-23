use std::sync::mpsc::channel;

#[test]
fn it_works() {
	let server = Route::new()
	.start("127.0.0.1:8081");
	
	panic!("Fail");
}

extern crate hyper;

use std::net::ToSocketAddrs;
use hyper::net::HttpListener;
use std::collections::HashMap;
use hyper::net::Fresh;
use hyper::status::StatusCode;
use std::io::{Read, Write};

struct Request;
struct Response<'a>{
	hyper: hyper::server::response::Response<'a, Fresh>
}
impl<'a> Response<'a>{
	fn send<T>(mut self, data: T) where T: Body{
		let mut stream = self.hyper.start().unwrap();
		data.write_to(&mut stream);
	}
}

trait Handler: Send + Sync{
	fn handle(&self, req: Request, res: Response);
}

struct Server{
	listening: hyper::server::Listening
}

struct RootHandler{
	handlers: HashMap<String, Box<Handler>>
}
impl RootHandler{
	fn new() -> RootHandler{
		RootHandler{
			handlers: HashMap::new()
		}
	}
}

struct Route{
	sub_handlers: HashMap<String, Route>,
	root_handler: RootHandler
}
unsafe impl Send for Route{}
unsafe impl Sync for Route{}
impl Handler for Route{
	fn handle(&self, req: Request, mut res: Response){
		res.send("Hello from wormhole!")
	}
}

trait Body{
	fn write_to(&self, dest: &mut Write);
}
impl<'a> Body for &'a str{
	fn write_to(&self, dest: &mut Write){
		dest.write(self.as_bytes()).unwrap();
	}
}

impl Route{
	fn new() -> Route{
		Route{
			sub_handlers: HashMap::new(),
			root_handler: RootHandler::new()
		}
	}
	
	
	fn start<A>(self, addr: A) -> Server
	where A: ToSocketAddrs{
		let server = hyper::Server::http(addr).unwrap();
		let listening = server.handle(move |req: hyper::server::request::Request, mut res: hyper::server::response::Response<Fresh>|{
			{
				let mut request = Request;
				let mut response = Response{
					hyper: res
				};
				self.handle(request, response);
			}
			//println!("HANDLE STUFF");
			//*res.status_mut() = StatusCode::Ok;
			//let mut res2 = res.start().unwrap();
			//write!(res2, "test");
			//res2.end().unwrap();
			//res.send(b"Hello World!");
		}).unwrap();
		Server{
			listening: listening
		}
	}
}