use method::Method;
use hyper;
use rustc_serialize::json;
use rustc_serialize::Decodable;
use std::io::Read;
use body::Body;
use status_code::StatusCode;
use handler::{HandlerResult, Action};
use header::{Header, HeaderFormat};

use hyper::server::request::Request as HyperRequest;
use hyper::server::response::Response as HyperResponse;


pub struct Request<'a, 'b: 'a, 'c, 'd>{
	path: String,
	req: hyper::server::request::Request<'a, 'b>,
	res: hyper::server::response::Response<'c, hyper::net::Fresh>,
	cookie_key: &'d [u8]
}

impl<'a, 'b, 'c, 'd> Request<'a, 'b, 'c, 'd>{
	pub fn get_path(&self) -> &str{
		&self.path
	}
	pub fn set_path(&mut self, path: &str){
		self.path = path.to_owned();
	}
	pub fn get_original_path(&self) -> String{
		match self.req.uri{
			hyper::uri::RequestUri::AbsolutePath(ref path) => path,
			_ => panic!("not implemented: wrong RequestUri")
		}.clone()
	}
	pub fn get_method(&self) -> &Method{
		&self.req.method
	}
	pub fn get_json<T>(&mut self) -> Result<T, json::DecoderError>
	where T: Decodable{
		let buffer = &mut String::new();
		
		
		match self.req.read_to_string(buffer){
			Ok(_) => json::decode(buffer),
			Err(err) => Err(json::DecoderError::ParseError(json::ParserError::IoError(err))) 
		}
	}
	
	//pub fn new(hyper_res: hyper::server::response::Response<'a, hyper::net::Fresh>) -> Response<'a>{
	//	Response{
	//		hyper: hyper_res
	//	}
	//}
	
	pub fn send<E, T>(&mut self, status: StatusCode, body: T) -> HandlerResult<E>
	where T: Body + 'static{
		Ok(Action::Done( (status, Box::new(body)) ))
	}
	
	pub fn get_request_header<H>(&self) -> Option<&H>
	where H: Header + HeaderFormat{
		self.req.headers.get()
	}
	pub fn set_response_header<H>(&mut self, header: H)
	where H: Header + HeaderFormat{
		self.res.headers_mut().set(header);
	}
	
	pub fn next<E>(&self) -> HandlerResult<E>{
		Ok(Action::Next)
	}
	
	pub fn error<E>(&self, err: E) -> HandlerResult<E>{
		Err(err)
	}
}

pub fn new<'a, 'b, 'c, 'd>(req: hyper::server::request::Request<'a, 'b>, res: hyper::server::response::Response<'c, hyper::net::Fresh>, cookie_key: &'d [u8]) -> Request<'a, 'b, 'c, 'd>{
	let path:String = match req.uri{
		hyper::uri::RequestUri::AbsolutePath(ref path) => path,
		_ => panic!("not implemented: wrong RequestUri")
	}.clone();
	
	Request{
		path: path,
		req: req,
		res: res,
		cookie_key: cookie_key
	}	
}

pub fn deconstruct<'a, 'b: 'a, 'c, 'd>(req: Request<'a, 'b, 'c, 'd>) -> (HyperRequest<'a, 'b>, HyperResponse<'c, hyper::net::Fresh>){
	(req.req, req.res)
}