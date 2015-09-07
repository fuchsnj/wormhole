use route::sub_route::SubRoute;
use route::route::Route;
use route::after_route::AfterRoute;
use std::collections::HashMap;
use method::Method;
use handler::{Handler, NormalHandler};
use std::sync::Arc;
use route::{AddRootHandler, AddSubHandler, AddAfterMiddleware};


#[derive(Clone)]
pub struct RootRoute<E: 'static>{
	prev: Route<E>,
	
}

impl<E> RootRoute<E>{
	pub fn new(prev: Route<E>) -> RootRoute<E>{
		RootRoute{
			prev: prev,
			root_handlers: HashMap::new()
		}
	}
}



impl<E> AddSubHandler<E> for RootRoute<E>{
	fn add_sub_handler<T>(mut self, path:&str, handler: T) -> SubRoute<E>
	where T: NormalHandler<E> + 'static{
		SubRoute::new(self).add_sub_handler(path, handler)
	}
}

impl<E> AddAfterMiddleware<E> for RootRoute<E>{
	fn add_after_middleware<T>(mut self, handler: T) -> AfterRoute<E>
	where T: Handler<E> + 'static{
		SubRoute::new(self).add_after_middleware(handler)
	}
}

impl<E> NormalHandler<E> for RootRoute<E>{
	fn handle(&self, req: &mut Request, mut res: &mut Response) -> HandlerResult<E>{
		try!(self.prev.handle(req, res));
		
	}
}