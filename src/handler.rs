use request::Request;
use body::Body;
use status_code::StatusCode;

use server;
use server::Server;
use hyper;
use request;

pub type HandlerResult<E> = Result<Action, E>;

pub enum Action{
	Next,
	Done((StatusCode, Box<Body>))
}

pub trait Handler<E, P>: Send + Sync{
	fn handle<'a, 'b, 'c>(&self, req: &mut Request<'a, 'b, 'c, P>) -> HandlerResult<E>;
}

pub trait ErrorHandler<E, P>: Send + Sync{
	fn handle_error<'a, 'b, 'c>(&self, req: &mut Request<'a, 'b, 'c, P>, error: E) -> HandlerResult<E>{
		req.error(error)
	}
}
/*
pub trait ParamHandler<D1, D2, E>: Send + Sync{
	fn handle_param<'a, 'b, 'c>(&self, req: &mut Request<'a, 'b, 'c>, data: &D1, param: &str) -> Result<D2, E>;
}*/
pub trait PathHandler<E, P>: Send + Sync{
	fn handle_path<'a, 'b, 'c>(&self, req: &mut Request<'a, 'b, 'c, P>, param: Option<&str>) -> HandlerResult<E>;
}
/*
pub trait DataHandler<D1, D2, E>: Send + Sync{
	fn handle_data(&self, req: &mut Request, data: &D1) -> Result<D2, E>;
}*/

impl<E, P> Handler<E, P> for (){
	fn handle<'a, 'b, 'c>(&self, req: &mut Request<'a, 'b, 'c, P>) -> HandlerResult<E>{
		req.next()
	}	
} 

impl<T, E, P> Handler<E, P> for T
where T: Send + Sync + for<'a, 'b, 'c> Fn(&mut Request<'a, 'b, 'c, P>) -> HandlerResult<E> {
	fn handle<'a, 'b, 'c>(&self, req: &mut Request<'a, 'b, 'c, P>) -> HandlerResult<E>{
		(*self)(req)
	}
}


impl<E, P> ErrorHandler<E, P> for (){}

impl<T, E, P> ErrorHandler<E, P> for T
where T: Send + Sync + for<'a, 'b, 'c> Fn(&mut Request<'a, 'b, 'c, P>, E) -> HandlerResult<E> {
	fn handle_error<'a, 'b, 'c>(&self, req: &mut Request<'a, 'b, 'c, P>, err: E) -> HandlerResult<E>{
		(*self)(req, err)
	}
}

/*
impl<T, D1, D2, E> ParamHandler<D1, D2, E> for T
where T: Send + Sync + for<'a, 'b, 'c> Fn(&mut Request<'a, 'b, 'c>, &D1, &str) -> Result<D2, E> {
	fn handle_param<'a, 'b, 'c>(&self, req: &mut Request<'a, 'b, 'c>, data: &D1, param: &str) -> Result<D2, E>{
		(*self)(req, param)
	}
}*/

impl<T, E, P> PathHandler<E, P> for T
where T: Send + Sync + for<'a, 'b, 'c> Fn(&mut Request<'a, 'b, 'c, P>, Option<&str>) -> HandlerResult<E> {
	fn handle_path<'a, 'b, 'c>(&self, req: &mut Request<'a, 'b, 'c, P>, param: Option<&str>) -> HandlerResult<E>{
		(*self)(req, param)
	}
}
/*
impl<T, D1, D2, E> DataHandler<D1, D2, E> for T
where T: Send + Sync + for<'a, 'b, 'c> Fn(&mut Request<'a, 'b, 'c>, &D1) -> Result<D2, E> {
	fn handle_data<'a, 'b, 'c>(&self, req: &mut Request<'a, 'b, 'c>, data: &D1) -> Result<D2, E>{
		(*self)(req)
	}
}*/
