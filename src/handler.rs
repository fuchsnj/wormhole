use request::Request;
use body::Body;
use status_code::StatusCode;

use server;
use server::Server;
use hyper;
use request;

pub type HandlerResult<D, E> = Result<Action<D>, E>;

pub enum Action<D>{
	Next(D),
	Done((StatusCode, Box<Body>))
}

pub trait Handler<D1, D2, E>: Send + Sync{
	fn handle<'a, 'b, 'c, 'd>(&self, req: &mut Request<'a, 'b, 'c, 'd>, data: D1) -> HandlerResult<D2, E>;
}

pub trait ErrorHandler<D2, E>: Send + Sync{
	fn handle_error<'a, 'b, 'c, 'd>(&self, req: &mut Request<'a, 'b, 'c, 'd>, error: E) -> HandlerResult<D2, E>{
		req.error(error)
	}
}
pub trait ParamHandler<D1, D2, E>: Send + Sync{
	fn handle_param<'a, 'b, 'c, 'd>(&self, req: &mut Request<'a, 'b, 'c, 'd>, data: D1, param: &str) -> HandlerResult<D2, E>;
}
pub trait PathHandler<D1, E>: Send + Sync{
	fn handle_path<'a, 'b, 'c, 'd>(&self, req: &mut Request<'a, 'b, 'c, 'd>, data: D1, param: Option<&str>) -> HandlerResult<D1, E>;
}

impl<D1, E> Handler<D1, D1, E> for (){
	fn handle<'a, 'b, 'c, 'd>(&self, req: &mut Request<'a, 'b, 'c, 'd>, data: D1) -> HandlerResult<D1, E>{
		req.next(data)
	}	
} 

impl<T, D1, D2, E> Handler<D1, D2, E> for T
where T: Send + Sync + for<'a, 'b, 'c, 'd> Fn(&mut Request<'a, 'b, 'c, 'd>, D1) -> HandlerResult<D2, E> {
	fn handle<'a, 'b, 'c, 'd>(&self, req: &mut Request<'a, 'b, 'c, 'd>, data: D1) -> HandlerResult<D2, E>{
		(*self)(req, data)
	}
}


impl<D2, E> ErrorHandler<D2, E> for (){}

impl<T, D2, E> ErrorHandler<D2, E> for T
where T: Send + Sync + for<'a, 'b, 'c, 'd> Fn(&mut Request<'a, 'b, 'c, 'd>, E) -> HandlerResult<D2, E> {
	fn handle_error<'a, 'b, 'c, 'd>(&self, req: &mut Request<'a, 'b, 'c, 'd>, err: E) -> HandlerResult<D2, E>{
		(*self)(req, err)
	}
}

impl<T, D1,D2,  E> ParamHandler<D1, D2, E> for T
where T: Send + Sync + for<'a, 'b, 'c, 'd> Fn(&mut Request<'a, 'b, 'c, 'd>, D1, &str) -> HandlerResult<D2, E> {
	fn handle_param<'a, 'b, 'c, 'd>(&self, req: &mut Request<'a, 'b, 'c, 'd>, data: D1, param: &str) -> HandlerResult<D2, E>{
		(*self)(req, data, param)
	}
}

impl<T, D1, E> PathHandler<D1, E> for T
where T: Send + Sync + for<'a, 'b, 'c, 'd> Fn(&mut Request<'a, 'b, 'c, 'd>, D1, Option<&str>) -> HandlerResult<D1, E> {
	fn handle_path<'a, 'b, 'c, 'd>(&self, req: &mut Request<'a, 'b, 'c, 'd>, data: D1, param: Option<&str>) -> HandlerResult<D1, E>{
		(*self)(req, data, param)
	}
}
