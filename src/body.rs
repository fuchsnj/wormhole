use std::io::Write;
use std::io;
use serde_json::value::Value as Json;
use serde_json::ser;

pub trait Body{
	fn write_to(&self, dest: &mut Write) -> Result<(), io::Error>;
}
impl<'a> Body for &'a str{
	fn write_to(&self, dest: &mut Write) -> Result<(), io::Error>{
		dest.write_all(self.as_bytes())
	}
}

impl Body for String{
	fn write_to(&self, dest: &mut Write) -> Result<(), io::Error>{
		dest.write_all(self.as_bytes())
	}
}

impl Body for Json{
	fn write_to(&self, dest: &mut Write) -> Result<(), io::Error>{
		dest.write_all(ser::to_string(self).unwrap().as_bytes())
	}
}

impl<'a, T> Body for &'a T
where T : Body{
	fn write_to(&self, dest: &mut Write) -> Result<(), io::Error>{
		(*self).write_to(dest)
	}
}