use std::fs::File;
use std::io::Write;
use crate::column_spec::ColumnSpec;
use crate::row_writer::RowWriter;

pub struct DelimitedOutput<'a> {
    delimiter: String,
    specs: &'a Vec<ColumnSpec>,
    writer: File
}

impl<'a> DelimitedOutput<'a> {
    pub fn new(delimiter: String, specs: &'a Vec<ColumnSpec>, writer: File) -> Self {
        Self { delimiter, specs, writer }
    }
}

impl RowWriter for DelimitedOutput<'_> {
    fn begin(&mut self) {
        let header_line = self
            .specs
            .iter()
            .map(|col| col.name.to_string())
            .collect::<Vec<String>>()
            .join(&self.delimiter);

        writeln!(self.writer, "{}", header_line).unwrap();
    }

    fn write_row(&mut self, items: Vec<String>) {
        // I thought about trimming upstream but giving the output handlers flexibility is good. Make it configurable.
        let joined = items
            .iter()
            .map(|col| col.as_str().trim())
            .collect::<Vec<&str>>()
            .join(&self.delimiter);

        writeln!(self.writer, "{}", joined).unwrap();
    }

    fn end(&mut self) {
        let _ = self.writer.flush().unwrap();
    }
}