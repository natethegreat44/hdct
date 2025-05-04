use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};

/// Handy function to get input from a file if one was supplied, otherwise get input from stdin.
pub fn reader_from_file_or_stdin(filename: Option<String>) -> (u64, Box<dyn BufRead>) {
    match filename {
        None => (0, Box::new(BufReader::new(io::stdin().lock()))),
        Some(filename) => (
            std::fs::metadata(&filename)
                .expect("Failed to read metadata")
                .len(),
            Box::new(BufReader::new(
                File::open(filename).expect("Failed to open input file"),
            )),
        ),
    }
}

pub fn writer_to_file_or_stdout(filename: Option<String>) -> Box<dyn Write> {
    match filename {
        Some(name) => Box::new(File::create(name).expect("Failed to create output file")),
        None => Box::new(io::stdout().lock()),
    }
}
