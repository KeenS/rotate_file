use std::io;
use std::path::Path;

pub trait NewName {
    fn new_name<'a>(&mut self, &'a Path) -> io::Result<&'a Path>;
}


pub struct ConstantNewName;

impl NewName for ConstantNewName {
    fn new_name<'a>(&mut self, f: &'a Path) -> io::Result<&'a Path> {
        Ok(f)
    }
}
