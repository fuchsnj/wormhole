use handler::NormalHandler;
use route::SubRoute;

pub trait AddSubMiddleware<D1, D2, E>{
	fn add_sub_middleware<T>(mut self, path: &str, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static;
}

pub trait SubMiddlewareMethods<D1, D2, E>{
	fn route<T>(mut self, path: &str, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static;
}

impl<A, D1, D2, E> SubMiddlewareMethods<D1, D2, E> for A where A: AddSubMiddleware<D1, D2, E>{
	fn route<T>(mut self, path: &str, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static{
		self.add_sub_middleware(path, handler)
	}
}