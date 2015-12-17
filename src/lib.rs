extern crate hyper;
extern crate url;
extern crate rustc_serialize;
extern crate unicase;
extern crate openssl;
extern crate cookie;

pub mod handler;
mod route;
mod request;
mod body;
mod server;

mod status_code{
	pub use hyper::status::StatusCode;
}
mod method{
	pub use hyper::method::Method;
}
pub mod header{
	pub use hyper::header::*;
}
pub use route::Route;
pub use request::Request;
pub use status_code::StatusCode;
pub use method::Method;
pub use header::Header;

pub mod prelude{
	pub use route::route;
	pub use {Request, StatusCode};
	pub use Route;
	pub use server::Server;
	pub use header;
	pub use handler::{Action, HandlerResult};
	pub use cookie::Cookie;
}

