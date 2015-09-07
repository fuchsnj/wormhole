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


pub struct Request<'a, 'b: 'a, 'c>{
	path: String,
	req: hyper::server::request::Request<'a, 'b>,
	res: hyper::server::response::Response<'c, hyper::net::Fresh>
}

impl<'a, 'b, 'c> Request<'a, 'b, 'c>{
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
	
	pub fn send<E, D2, T>(&mut self, status: StatusCode, body: T) -> HandlerResult<D2, E>
	where T: Body + 'static{
		Ok(Action::Done( (status, Box::new(body)) ))
	}
	
	pub fn set_response_header<H>(&mut self, header: H)
	where H: Header + HeaderFormat{
		self.res.headers_mut().set(header);
	}
	
	pub fn next<D, E>(&self, data: D) -> HandlerResult<D, E>{
		Ok(Action::Next(data))
	}
	
	pub fn error<D, E>(&self, err: E) -> HandlerResult<D, E>{
		Err(err)
	}
}

pub fn new<'a, 'b, 'c>(req: hyper::server::request::Request<'a, 'b>, res: hyper::server::response::Response<'c, hyper::net::Fresh>) -> Request<'a, 'b, 'c>{
	let path:String = match req.uri{
		hyper::uri::RequestUri::AbsolutePath(ref path) => path,
		_ => panic!("not implemented: wrong RequestUri")
	}.clone();
	
	Request{
		path: path,
		req: req,
		res: res
	}	
}

pub fn deconstruct<'a, 'b: 'a, 'c>(req: Request<'a, 'b, 'c>) -> (HyperRequest<'a, 'b>, HyperResponse<'c, hyper::net::Fresh>){
	(req.req, req.res)
}