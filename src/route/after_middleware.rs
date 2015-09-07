use handler::{Handler, ErrorHandler, NormalHandler};
use route::AfterRoute;

pub trait AddAfterMiddleware<D1, D2, E>{
	fn add_after_middleware<T>(mut self, handler: T) -> AfterRoute<D1, D2, E>
	where T: Handler<D1, D2, E> + 'static;
}

pub trait AfterMiddlewareMethods<D1, D2, E>{
	fn using<T>(mut self, handler: T) -> AfterRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static;
	
	//fn catch<T>(mut self, handler: T) -> AfterRoute<D1, D2, E>
	//where T: ErrorHandler<D2, E> + 'static;
}

impl<A, D1, D2, E> AfterMiddlewareMethods<D1, D2, E> for A where A: AddAfterMiddleware<D1, D2, E>{
	fn using<T>(mut self, normal_handler: T) -> AfterRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static{
		let handler = (normal_handler, () );
		self.add_after_middleware(handler)
	}
	
	//fn catch<T>(mut self, error_handler: T) -> AfterRoute<D1, D2, E>
	//where T: ErrorHandler<D2, E> + 'static{
	//	let handler = ( (), error_handler);
	//	self.add_after_middleware(handler)
	//}
}