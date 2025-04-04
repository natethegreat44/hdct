use std::io::Write;
use crate::column_spec::ColumnSpec;
use crate::row_writer::RowWriter;

pub struct DelimitedOutput<'a> {
    delimiter: String,
    specs: &'a Vec<ColumnSpec>
}

impl<'a> DelimitedOutput<'a> {
    pub fn new(delimiter: String, specs: &'a Vec<ColumnSpec>) -> Self {
        Self { delimiter, specs }
    }
}

impl RowWriter for DelimitedOutput<'_> {
    fn begin(&self) {
        let header_line = self
            .specs
            .iter()
            .map(|col| col.name.to_string())
            .collect::<Vec<String>>()
            .join(&self.delimiter);

        println!("{}", header_line);
    }

    fn write_row(&self, items: Vec<String>) {
        let joined = items.join(&self.delimiter);
        println!("{}", joined);
    }

    fn end(&self) {
        // self.writer.flush().unwrap();
    }
}
