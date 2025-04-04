use std::io::Write;
use crate::column_spec::ColumnSpec;
use crate::row_writer::RowWriter;

pub struct DelimitedOutput<'a> {
    delimiter: String,
    specs: &'a Vec<ColumnSpec>,
    writer: Box<dyn Write>
}

impl<'a> DelimitedOutput<'a> {
    pub fn new(delimiter: String, specs: &'a Vec<ColumnSpec>, writer: Box<dyn Write>) -> Self {
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

        let line = format!("{header_line}\n");
        let _ = self.writer.write_all(line.as_bytes());
    }

    fn write_row(&mut self, items: Vec<String>) {
        let joined = items.join(&self.delimiter);
        let line = format!("{joined}\n");
        let _ = self.writer.write_all(line.as_bytes());
    }

    fn end(&mut self) {
        let _ = self.writer.flush().unwrap();
    }
}