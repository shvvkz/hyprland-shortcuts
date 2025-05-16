use std::fs;
use std::io;
use std::path::Path;

pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}
