// use std::fs::File;
// use std::io::{self, Write};
//
// pub struct OutputHandler {
//     writer: Result<Box<dyn Write>, io::Error>,
// }
//
// fn get_writer(filename: Option<String>) -> Result<Box<dyn Write>, io::Error> {
//     match filename {
//         Some(name) => Ok(Box::new(File::create(name)?)),
//         None => Ok(Box::new(io::stdout()))
//     }
// }
//
// impl OutputHandler {
//     pub fn new(filename: Option<String>) -> Self {
//         OutputHandler {
//             writer: get_writer(filename),
//         }
//     }
//
//     pub fn write_data(&mut self, data: &str) -> Result<(), io::Error> {
//         match &mut self.writer {
//             Ok(writer) => writer.write_all(data.as_bytes()),
//             Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("Writer not available: {}", e))),
//         }
//     }
//
//     pub fn flush(&mut self) -> Result<(), io::Error> {
//         match &mut self.writer {
//             Ok(writer) => writer.flush(),
//             Err(_) => Ok(()), // Nothing to flush if there's no writer
//         }
//     }
// }
