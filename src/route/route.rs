use handler::{Handler, NormalHandler, HandlerResult, Action};
use route::sub_route::SubRoute;
use route::after_route::AfterRoute;
use std::sync::Arc;
use route::root_middleware::AddRootMiddleware;
use route::before_middleware::AddBeforeMiddleware;
use route::sub_middleware::AddSubMiddleware;
use route::after_middleware::AddAfterMiddleware;
use method::Method;
use request::Request;

#[derive(Clone)]
pub struct Route<D1, D2, E: 'static>{
	before_middleware: Vec<Arc<Handler<D1, D2, E>>>
}

impl<D1, D2, E> Route<D1, D2, E>{
	pub fn new() -> Route<D1, D2, E>{
		Route{
			before_middleware: Vec::new()
		}
	}
}

impl<D1, D2, E> AddBeforeMiddleware<D1, D2, E> for Route<D1, D2, E>{
	fn add_before_middleware<T>(mut self, handler: T) -> Route<D1, D2, E>
	where T: Handler<D1, D2, E> + 'static{
		self.before_middleware.push(Arc::new(handler));
		self
	}
}
impl<D1, D2, E> AddRootMiddleware<D1, D2, E> for Route<D1, D2, E>{
	fn add_root_middleware<T>(mut self, method: Method, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static{
		SubRoute::new(self).add_root_middleware(method, handler)
	}
}

impl<D1, D2, E> AddSubMiddleware<D1, D2, E> for Route<D1, D2, E>{
	fn add_sub_middleware<T>(mut self, path:&str, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static{
		SubRoute::new(self).add_sub_middleware(path, handler)
	}
}

impl<D1, D2, E> NormalHandler<D1, D2, E> for Route<D1, D2, E>{
	fn handle(&self, req: &mut Request, data: D1) -> HandlerResult<D2, E>{
		panic!("need to implement route.rs handler");
	/*
		let mut current_error: Option<E> = None;
		
		for ref handler in &self.before_middleware{
			let result = match current_error{
				Some(err) => handler.error(req, err),
				None => handler.handle(req, data)
			};
			match result{
				Ok(Action::Next(data)) => {
					current_error = None;
				},
				Err(err) => {
					current_error = Some(err);
				},
				Ok(Action::Done(data)) => {
					return Ok(Action::Done(data));
				}
			}
		}
		
		match current_error{
			Some(err) => Err(err),
			None => req.next()
		}
		*/
	}
}
