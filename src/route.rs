use handler::{HandlerResult, Handler, ErrorHandler, PathHandler, Action};
use request::Request;

use server::Server;
use hyper;
use status_code::StatusCode;
use request;
use server;
use body::Body;
use std::marker::PhantomData;
use std::sync::Arc;
use method::Method;
use header;
use unicase::UniCase;
use std::fmt;

pub struct Route<E, P>{
	handler: Arc<Handler<E, P> + Send + Sync + 'static>
}

pub fn route<E, P>() -> Route<E, P>{
	Route{
		handler: Arc::new(())
	}
}

impl<E: 'static, P> Route<E, P>{
	/*
	pub fn using<H2: 'static>(self, handler: H2) -> Route<E>
	where H2: Send + Sync + Handler<E>{
		Route{
			handler: Arc::new(move |req: &mut Request|{
				match self.handler.handle(req){
					Ok(Action::Next) => {},
					Ok(Action::Done(res)) => return Ok(Action::Done(res)),
					Err(err) => return Err(err)
				};
				handler.handle(req)
			})
		}
	}*/
	/*
	pub fn data<DH, H, D2>(self, data_handler: DH, handler: H) -> Route<D, E> where
	DH: Send + Sync + DataHandler<D, D2, E> + 'static,
	H: Send + Sync + Handler<D2, E> + 'static{
		Route{
			handler: Arc::new(move |req: &mut Request, data: &D|{
				match self.handler.handle(req, data){
					Ok(Action::Next) => {},
					Ok(Action::Done(res)) => return Ok(Action::Done(res)),
					Err(err) => return Err(err)
				};
				let data2: D2 = match data_handler.handle_data(req, data){
					Ok(data) => data,
					Err(err) => return Err(err)
				};
				match handler.handle(req, &data2){
					Ok(Action::Next) => Ok(Action::Next),
					Ok(Action::Done(res)) => Ok(Action::Done(res)),
					Err(err) => Err(err)
				}
			})
		}
	}*/
	/*
	pub fn catch<H2: 'static>(self, handler: H2) -> Route<E>
	where H2: Send + Sync + ErrorHandler<E>{
		Route{
			handler: Arc::new(move |req: &mut Request|{
				let err:E = match self.handler.handle(req){
					Ok(Action::Next) => return Ok(Action::Next),
					Ok(Action::Done(res)) => return Ok(Action::Done(res)),
					Err(err) => err
				};
				handler.handle_error(req, err)
			})
		}
	}*/
	/*
	pub fn param<H2, H3, D2>(self, param_handler: H2, handler: H3) -> Route<D, E> where
	H2: Send + Sync + ParamHandler<D, D2, E> + 'static,
	H3: Send + Sync + Handler<D2, E> + 'static{
		self.path(move |req: &mut Request, data: &D, path: Option<&str>|{
			match path{
				Some(param) => {
					let data2: D2 = match param_handler.handle_param(req, data, param){
						Ok(data) => data,
						Err(err) => return Err(err)
					};
					match handler.handle(req, &data2){
						Ok(Action::Next) => Ok(Action::Next),
						Ok(Action::Done(res)) => Ok(Action::Done(res)),
						Err(err) => Err(err)
					}
				},
				None => req.next()
			}
		})
	}*/
	/*
	pub fn route<H2>(self, path: &str, handler: H2) -> Route<E>
	where H2: Send + Sync + Handler<E> + 'static{
		let path = path.to_owned();
		self.path(move |req: &mut Request, p: Option<&str>|{
			match p{
				Some(param) => {
					match param == path{
						true => handler.handle(req),
						false => req.next()
					}
				},
				None => req.next()
			}
		})
	}*/
	/*
	pub fn root<H2>(self, handler: H2) -> Route<E>
	where H2: Send + Sync + Handler<E> + 'static{
		self.path(move |req: &mut Request, path: Option<&str>|{
			match path{
				Some(_) => req.next(),
				None => handler.handle(req)
			}
		})
	}*/
	/*
	pub fn path<H2>(self, handler: H2) -> Route<E>
	where H2: Send + Sync + PathHandler<E> + 'static{
		Route{
			handler: Arc::new(move |req: &mut Request|{
				match self.handler.handle(req){
					Ok(Action::Next) => {},
					Ok(Action::Done(res)) => return Ok(Action::Done(res)),
					Err(err) => return Err(err)
				};
				let path:String = req.get_path().to_owned();
				let (segment, path_remainder) = get_next_url_segment(&path);
				match segment{
					Some(segment) => {
						req.set_path(path_remainder);
						let result = handler.handle_path(req, Some(segment));
						req.set_path(&path);
						match result{
							Ok(Action::Next) => Ok(Action::Next),
							Ok(Action::Done(res)) => Ok(Action::Done(res)),
							Err(err) => Err(err)
						}
					},
					None => handler.handle_path(req, None)
				}
			})
		}
	}*/
	/*
	pub fn method<H2>(self, method: Method, handler: H2) -> Route<E>
	where H2: Send + Sync + Handler<E> + 'static{
		self.root(move |req: &mut Request|{
			match req.get_method() == &method{
				true => handler.handle(req),
				false => req.next()
			}
		})
	}*/
	/*
	pub fn get<H2>(self, handler: H2) -> Route<E>
	where H2: Handler<E> + 'static{
		self.method(Method::Get, handler)
	}
	pub fn post<H2>(self, handler: H2) -> Route<E>
	where H2: Handler<E> + 'static{
		self.method(Method::Post, handler)
	}
	pub fn put<H2>(self, handler: H2) -> Route<E>
	where H2: Handler<E> + 'static{
		self.method(Method::Put, handler)
	}
	pub fn delete<H2>(self, handler: H2) -> Route<E>
	where H2: Handler<E> + 'static{
		self.method(Method::Delete, handler)
	}
	pub fn head<H2>(self, handler: H2) -> Route<E>
	where H2: Handler<E> + 'static{
		self.method(Method::Head, handler)
	}
	pub fn trace<H2>(self, handler: H2) -> Route<E>
	where H2: Handler<E> + 'static{
		self.method(Method::Trace, handler)
	}
	pub fn connect<H2>(self, handler: H2) -> Route<E>
	where H2: Handler<E> + 'static{
		self.method(Method::Connect, handler)
	}
	pub fn patch<H2>(self, handler: H2) -> Route<E>
	where H2: Handler<E> + 'static{
		self.method(Method::Patch, handler)
	}
	pub fn options<H2>(self, handler: H2) -> Route<E>
	where H2: Handler<E> + 'static{
		self.method(Method::Options, handler)
	}*/
	/*
	pub fn cors(self) -> Route<E>{
		self.using(|req: &mut Request| -> HandlerResult<E>{
			if let Some(origin) = req.get_request_header::<OriginHeader>()
			.map(|h|h.clone()){
				req.set_response_header(header::AccessControlAllowOrigin::Value(origin.0.clone()));
			}
			if let Some(requested_headers) = req.get_request_header::<header::AccessControlRequestHeaders>()
			.map(|h|h.0.clone()){
				req.set_response_header(header::AccessControlAllowHeaders(requested_headers));
			}
			req.set_response_header(AccessControlAllowCredentialsHeader);
			
			if req.get_method() == &Method::Options {
				req.send(StatusCode::Ok, "")
			}else{
				req.next()
			}
		})
	}*/
}

// Temporary implementation of Origin header until Hyper has an official one
// https://github.com/hyperium/hyper/issues/651
#[derive(Clone, Debug)]
struct OriginHeader(String);
impl hyper::header::Header for OriginHeader{
	fn header_name() -> &'static str{
		"Origin"
	}
	fn parse_header(raw: &[Vec<u8>]) -> Result<Self, hyper::error::Error>{
		Ok(OriginHeader(
			try!(String::from_utf8(raw[0].clone()))
		))
	}
}
impl hyper::header::HeaderFormat for OriginHeader{
	 fn fmt_header(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>{
	 	Ok(())
	 }
}

// Temporary implementation of AccessControlAllowCredentials header until Hyper has an official one
// https://github.com/hyperium/hyper/issues/655
#[derive(Clone, Debug)]
struct AccessControlAllowCredentialsHeader;
impl hyper::header::Header for AccessControlAllowCredentialsHeader{
	fn header_name() -> &'static str{
		"Access-Control-Allow-Credentials"
	}
	fn parse_header(_: &[Vec<u8>]) -> Result<Self, hyper::error::Error>{
		Ok(AccessControlAllowCredentialsHeader)
	}
}
impl hyper::header::HeaderFormat for AccessControlAllowCredentialsHeader{
	fn fmt_header(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>{
		f.write_str("true")
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
			b'?' => match a == segment_start{
				true => return ( None, &path[a..] ),
				false => return ( Some(&path[segment_start..a]), &path[a..] )
			},
			_ => {}
		}
	}
	match path.len() > segment_start{
		true => ( Some(&path[segment_start..]), "" ),
		false => ( None, "" )
	}
}

impl<E, P> Handler<E, P> for Route<E, P>{
	fn handle(&self, req: &mut Request<P>) -> HandlerResult<E>{
		self.handler.handle(req)
	}
}