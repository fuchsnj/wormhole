use request::Request;
use body::Body;
use status_code::StatusCode;

pub type HandlerResult<E> = Result<Action, E>;

pub enum Action{
	Next,
	Done((StatusCode, Box<Body>))
}

pub trait Handler<D, E>: Send + Sync{
	fn handle<'a, 'b, 'c>(&self, req: &mut Request<'a, 'b, 'c>, data: D) -> HandlerResult<E>;
}

pub trait ErrorHandler<E>: Send + Sync{
	fn handle_error<'a, 'b, 'c>(&self, req: &mut Request<'a, 'b, 'c>, error: E) -> HandlerResult<E>{
		req.error(error)
	}
}
/*
pub trait ParamHandler<D1, D2, E>: Send + Sync{
	fn handle_param<'a, 'b, 'c>(&self, req: &mut Request<'a, 'b, 'c>, data: &D1, param: &str) -> Result<D2, E>;
}*/

pub trait PathHandler<D, E>: Send + Sync{
	fn handle_path<'a, 'b, 'c>(&self, req: &mut Request<'a, 'b, 'c>, data: D, param: Option<&str>) -> HandlerResult<E>;
}
/*
pub trait DataHandler<D1, D2, E>: Send + Sync{
	fn handle_data(&self, req: &mut Request, data: &D1) -> Result<D2, E>;
}*/

impl<D, E> Handler<D, E> for (){
	fn handle<'a, 'b, 'c>(&self, req: &mut Request<'a, 'b, 'c>, _: D) -> HandlerResult<E>{
		req.next()
	}	
} 

impl<T, D, E> Handler<D, E> for T
where T: Send + Sync + for<'a, 'b, 'c> Fn(&mut Request<'a, 'b, 'c>, D) -> HandlerResult<E> {
	fn handle<'a, 'b, 'c>(&self, req: &mut Request<'a, 'b, 'c>, data: D) -> HandlerResult<E>{
		(*self)(req, data)
	}
}


impl<E> ErrorHandler<E> for (){}

impl<T, E> ErrorHandler<E> for T
where T: Send + Sync + for<'a, 'b, 'c> Fn(&mut Request<'a, 'b, 'c>, E) -> HandlerResult<E> {
	fn handle_error<'a, 'b, 'c>(&self, req: &mut Request<'a, 'b, 'c>, err: E) -> HandlerResult<E>{
		(*self)(req, err)
	}
}
/*
impl<T, D1, D2, E> ParamHandler<D1, D2, E> for T
where T: Send + Sync + for<'a, 'b, 'c> Fn(&mut Request<'a, 'b, 'c>, &D1, &str) -> Result<D2, E> {
	fn handle_param<'a, 'b, 'c>(&self, req: &mut Request<'a, 'b, 'c>, data: &D1, param: &str) -> Result<D2, E>{
		(*self)(req, data, param)
	}
}*/

impl<T, D, E> PathHandler<D, E> for T
where T: Send + Sync + for<'a, 'b, 'c> Fn(&mut Request<'a, 'b, 'c>, D, Option<&str>) -> HandlerResult<E> {
	fn handle_path<'a, 'b, 'c>(&self, req: &mut Request<'a, 'b, 'c>, data: D, param: Option<&str>) -> HandlerResult<E>{
		(*self)(req, data, param)
	}
}
/*
impl<T, D1, D2, E> DataHandler<D1, D2, E> for T
where T: Send + Sync + for<'a, 'b, 'c> Fn(&mut Request<'a, 'b, 'c>, &D1) -> Result<D2, E> {
	fn handle_data<'a, 'b, 'c>(&self, req: &mut Request<'a, 'b, 'c>, data: &D1) -> Result<D2, E>{
		(*self)(req, data)
	}
}*/
