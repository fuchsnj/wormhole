use header;
use status_code::StatusCode;
use unicase::UniCase;
use method::Method;
use handler::{Handler, NormalHandler, ErrorHandler, HandlerResult};
use request::Request;
use route::Route;

pub trait AddBeforeMiddleware<D1, D2, E>{
	fn add_before_middleware<T>(mut self, handler: T) -> Route<D1, D2, E>
	where T: Handler<D1, D2, E> + 'static;
}

pub trait BeforeMiddlewareMethods<D1, D2, E>{
	fn using<T>(mut self, handler: T) -> Route<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static;
	
	//fn catch<T>(mut self, handler: T) -> Route<D1, D2, E>
	//where T: ErrorHandler<D2, E> + 'static;
	
	//fn cors(mut self) -> Route<D1, D2, E>;
}

impl<A, D1, D2, E> BeforeMiddlewareMethods<D1, D2, E> for A where A: AddBeforeMiddleware<D1, D2, E>{
	fn using<T>(mut self, normal_handler: T) -> Route<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static{
		let handler = (normal_handler, () );
		self.add_before_middleware(handler)
	}
	
	//fn catch<T>(mut self, error_handler: T) -> Route<D1, D2, E>
	//where T: ErrorHandler<D2, E> + 'static{
	//	let handler = ( (), error_handler);
	//	self.add_before_middleware(handler)
	//}
	
	//fn cors(mut self) -> Route<D1, D2, E>{
	//	self.using(cors_handler)
	//}
}


fn cors_handler<D1, E>(req: &mut Request, data: D1) -> HandlerResult<D1, E>{
	//println!("cors handler!");
	if req.get_method() == &Method::Options {
		//println!("options request!");
		req.set_response_header(header::AccessControlAllowOrigin::Any);
		req.set_response_header(header::AccessControlAllowHeaders(vec!(
			UniCase("Content-Type".to_owned())
		)));
		req.send(StatusCode::Ok, "")
	}else{
		req.set_response_header(header::AccessControlAllowOrigin::Any);
		req.next(data)
	}
}