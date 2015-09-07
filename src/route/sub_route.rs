use route::route::Route;
use handler::{Handler, NormalHandler, HandlerResult, Action};
use std::collections::HashMap;
use std::sync::Arc;
use route::sub_middleware::AddSubMiddleware;
use route::root_middleware::AddRootMiddleware;
use route::after_middleware::AddAfterMiddleware;
use route::after_route::AfterRoute;
use method::Method;
use request::Request;

#[derive(Clone)]
pub struct SubRoute<D1, D2, E: 'static>{
	prev: Route<D1, D2, E>,
	root_handlers: HashMap<Method, Arc<NormalHandler<D2, D2, E>>>,
	sub_handlers: HashMap<String, Arc<NormalHandler<D2, D2, E>>>
}

impl<D1, D2, E> SubRoute<D1, D2, E>{
	pub fn new(prev: Route<D1, D2, E>) -> SubRoute<D1, D2, E>{
		SubRoute{
			prev: prev,
			root_handlers: HashMap::new(),
			sub_handlers: HashMap::new()
		}
	}
}
impl<D1, D2, E> AddRootMiddleware<D1, D2, E> for SubRoute<D1, D2, E>{
	fn add_root_middleware<T>(mut self, method: Method, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static{
		self.root_handlers.insert(method, Arc::new(handler));
		self
	}
}

impl <D1, D2, E> AddSubMiddleware<D1, D2, E> for SubRoute<D1, D2, E>{
	fn add_sub_middleware<T>(mut self, path: &str, handler: T) -> SubRoute<D1, D2, E>
	where T: NormalHandler<D1, D2, E> + 'static{
		self.sub_handlers.insert(path.to_string(), Arc::new(handler));
		self
	}
}

impl<D1, D2, E> AddAfterMiddleware<D1, D2, E> for SubRoute<D1, D2, E>{
	fn add_after_middleware<T>(mut self, handler: T) -> AfterRoute<D1, D2, E>
	where T: Handler<D1, D2, E> + 'static{
		AfterRoute::new(self).add_after_middleware(handler)
	}
}

impl<D1, E> NormalHandler<D1, D1, E> for SubRoute<D1, D1, E>{
	fn handle(&self, req: &mut Request, data1: D1) -> HandlerResult<D1, E>{
		let data2:D2 = match self.prev.handle(req, data1){
			Ok(Action::Next(data)) => data,
			Ok(Action::Done(res)) => return Ok(Action::Done(res)),
			Err(err) => return Err(err)
		};
		
		
		let path:String = req.get_path().to_owned();
		let (segment, path_remainder) = get_next_url_segment(&path);
		match segment{
			None => {
				match self.root_handlers.get(req.get_method()){
					Some(handler) => handler.handle(req, data2),
					None => req.next(data2)
				}
			},
			Some(ref segment) => {
				match self.sub_handlers.get(&segment.to_string()){
					Some(handler) => {
						req.set_path(path_remainder);
						let result = handler.handle(req, data2);
						req.set_path(&path);
						result
					},
					None => req.next(data2)
				}
			}
		}
	}
}

fn get_next_url_segment(mut path: &str) -> (Option<&str>, &str){
	let mut segment_start = 0;
	for a in 0..path.len(){
		match path.as_bytes()[a]{
			b'/' | b'\\' => {
				match a{
					0 => segment_start+=1,
					_ => return ( Some(&path[segment_start..a]), &path[a..] )
				}
			},
			b'?' => match a{
				0 => return ( None, &path[a..] ),
				_ => return ( Some(&path[segment_start..a]), &path[a..] )
			},
			_ => {}
		}
	}
	match path.len() > segment_start{
		true => ( Some(&path[segment_start..]), "" ),
		false => ( None, "" )
	}
}