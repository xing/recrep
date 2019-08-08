use std::fs::File;
use std::io::prelude::*;

/// A Writing thing can write
pub trait Writing {
    /// Writes the text into something at the given path
    fn write(&self, text: String, path: String);
}

/// A FileWriter can write text to a file
pub struct FileWriter {}

impl Writing for FileWriter {
    /// Writes the text into a file at the given path
    fn write(&self, text: String, path: String) {
        let mut file = File::create(path).unwrap();
        let _ = file.write_all(text.as_bytes());
    }
}
