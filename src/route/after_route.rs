use route::sub_route::SubRoute;
use handler::{Action, Handler, NormalHandler, HandlerResult};
use std::sync::Arc;
use route::after_middleware::AddAfterMiddleware;
use request::Request;

#[derive(Clone)]
pub struct AfterRoute<D1, D2, E: 'static>{
	prev: SubRoute<D1, D2, E>,
	after_middleware: Vec<Arc<Handler<D1, D2, E>>>
}

impl<D1, D2, E> AfterRoute<D1, D2, E>{
	pub fn new(prev: SubRoute<D1, D2, E>) -> AfterRoute<D1, D2, E>{
		AfterRoute{
			prev: prev,
			after_middleware: Vec::new()
		}
	}
}

impl<D1, D2, E> AddAfterMiddleware<D1, D2, E> for AfterRoute<D1, D2, E>{
	fn add_after_middleware<T>(mut self, handler: T) -> AfterRoute<D1, D2, E>
	where T: Handler<D1, D2, E> + 'static{
		self.after_middleware.push(Arc::new(handler));
		self
	}
}

impl<D1, D2, E> NormalHandler<D1, D2, E> for AfterRoute<D1, D2, E>{
	fn handle(&self, req: &mut Request, data: D1) -> HandlerResult<D2, E>{
		panic!("need to implement after_route.rs");
	/*
		let mut current_error = match self.prev.handle(req){
				Ok(Action::Next) => None,
				Err(err) => Some(err),
				Ok(Action::Done(data)) => {
					return Ok(Action::Done(data));
				}
		};
		
		for ref handler in &self.after_middleware{
			let result = match current_error{
				Some(err) => handler.error(req, err),
				None => handler.handle(req, data)
			};
			match result{
				Ok(Action::Next) => {
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