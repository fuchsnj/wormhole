use hyper;

pub struct Server{
	listening: hyper::server::Listening
}
impl Server{
	pub fn stop(self){}
}

pub fn new(listening: hyper::server::Listening) -> Server{
	Server{
		listening: listening
	}
}
