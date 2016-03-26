use std::io;
use std::fs::File;
use std::path::Path;


pub trait RotationPolicy {
    fn need_rotate(&self, &Path, &File) -> io::Result<bool>;
}


pub struct SizeRotationPolicy {
    size: u64,
}


impl RotationPolicy for SizeRotationPolicy {
    fn need_rotate(&self, _path: &Path, file: &File) -> io::Result<bool> {
        let metadata = try!(file.metadata());
        Ok(self.size < metadata.len())
    }
}

