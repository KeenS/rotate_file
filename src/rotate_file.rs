use std::io::{Write, Result};
use std::fs::{File};
use std::path::Path;
use std::fs::rename;
use rotation_policy::*;
use new_name::*;

pub struct RotateFile<L, N> {
    inner: File,
    path: Box<Path>,
    rotation_policy: L,
    new_name: N
}

impl<L, N> RotateFile<L, N>
    where L: RotationPolicy,
          N: NewName
{
    pub fn new(path: Box<Path>, rotation_policy: L, new_name: N) -> Self {
        RotateFile {
            inner: File::create(path.as_ref()).unwrap(),
            path: path,
            rotation_policy: rotation_policy,
            new_name: new_name
        }
    }

    fn rotate(&mut self) -> Result<()> {
        try!(self.inner.flush());
        let &mut RotateFile{ref path, ref mut new_name, ..} = self;
        // TODO: be thread safe
        try!(rename(path.as_ref(), try!(new_name.new_name(path.as_ref()))));
        self.inner = try!(File::create(path));
        Ok(())
    }

    fn write_(&mut self, buf: &[u8]) -> Result<usize> {
        if try!(self.rotation_policy.need_rotate(&self.path, &self.inner)) {
            try!(self.rotate());
        }
        self.inner.write(buf)
    }

    fn flush_(&mut self) -> Result<()> {
        self.inner.flush()
    }
}

impl<L, N>  Write for RotateFile<L, N>
    where L: RotationPolicy,
          N: NewName {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.write_(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.flush_()
    }
}
