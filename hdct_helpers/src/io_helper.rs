use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

/// Handy function to get input from a file if one was supplied, otherwise get input from stdin.
pub fn reader_from_file_or_stdin(file_name: Option<String>) -> Box<dyn BufRead> {
    let reader: Box<dyn BufRead> = match file_name {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(File::open(filename).unwrap())),
    };

    reader
}
