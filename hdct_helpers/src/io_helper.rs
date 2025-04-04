use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};

/// Handy function to get input from a file if one was supplied, otherwise get input from stdin.
pub fn reader_from_file_or_stdin(filename: Option<String>) -> Box<dyn BufRead> {
    let reader: Box<dyn BufRead> = match filename {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(File::open(filename).unwrap())),
    };

    reader
}

pub fn writer_to_file_or_stdout(filename: Option<String>) -> Result<Box<dyn Write>, io::Error> {
    match filename {
        Some(name) => Ok(Box::new(File::create(name)?)),
        None => Ok(Box::new(io::stdout()))
    }
}