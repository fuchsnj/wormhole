use method::Method;
use route::SubRoute;
use handler::NormalHandler;

pub trait AddRootMiddleware<D1, D2, E>{
	fn add_root_middleware<T>(mut self, method: Method, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static;
}

pub trait RootMiddlewareMethods<D1, D2, E>{
	fn method<T>(mut self, method: Method, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static;
	
	fn get<T>(self, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static;
	
	fn post<T>(self, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static;
	
	fn put<T>(self, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static;
	
	fn delete<T>(self, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static;
	
	fn head<T>(self, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static;
	
	fn trace<T>(self, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static;
	
	fn connect<T>(self, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static;
	
	fn patch<T>(self, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static;
	
	fn options<T>(self, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static;
}

impl<A, D1, D2, E> RootMiddlewareMethods<D1, D2, E> for A where A: AddRootMiddleware<D1, D2, E>{
	fn method<T>(mut self, method: Method, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static{
		self.add_root_middleware(method, handler)
	}
	fn get<T>(self, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static{
		self.method(Method::Get, handler)
	}
	
	fn post<T>(self, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static{
		self.method(Method::Post, handler)
	}
	
	fn put<T>(self, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static{
		self.method(Method::Put, handler)
	}
	
	fn delete<T>(self, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static{
		self.method(Method::Delete, handler)
	}
	
	fn head<T>(self, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static{
		self.method(Method::Head, handler)
	}
	
	fn trace<T>(self, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static{
		self.method(Method::Trace, handler)
	}
	
	fn connect<T>(self, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static{
		self.method(Method::Connect, handler)
	}
	
	fn patch<T>(self, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static{
		self.method(Method::Patch, handler)
	}
	
	fn options<T>(self, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static{
		self.method(Method::Options, handler)
	}
}