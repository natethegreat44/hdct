use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};

/// Handy function to get input from a file if one was supplied, otherwise get input from stdin.
pub fn reader_from_file_or_stdin(filename: Option<String>) -> Box<dyn BufRead> {
    match filename {
        None => Box::new(BufReader::new(io::stdin().lock())),
        Some(filename) => Box::new(BufReader::new(File::open(filename).unwrap())),
    }
}

pub fn writer_to_file_or_stdout(filename: Option<String>) -> Box<dyn Write> {
    match filename {
        Some(name) => Box::new(File::create(name).unwrap()),
        None => Box::new(io::stdout().lock()),
    }
}
