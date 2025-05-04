use crate::column_spec::ColumnSpec;
use crate::row_writer::RowWriter;
use std::fs::File;
use std::io::{Result, Write};

pub struct DelimitedOutput<'a> {
    delimiter: String,
    specs: &'a Vec<ColumnSpec>,
    output_file: File,
}

impl<'a> DelimitedOutput<'a> {
    pub fn new(delimiter: String, specs: &'a Vec<ColumnSpec>, output_file: File) -> Self {
        Self {
            delimiter,
            specs,
            output_file,
        }
    }
}

impl RowWriter for DelimitedOutput<'_> {
    fn begin(&mut self) -> Result<()> {
        let header_line = self
            .specs
            .iter()
            .map(|col| col.name.to_string())
            .collect::<Vec<String>>()
            .join(&self.delimiter);

        writeln!(self.output_file, "{}", header_line).expect("Unable to write header to output");
        
        Ok(())
    }

    fn write_row(&mut self, items: Vec<String>) -> Result<()> {
        // I thought about trimming upstream but giving the output handlers flexibility is good. Make it configurable.
        let joined = items
            .iter()
            .map(|col| col.as_str().trim())
            .collect::<Vec<&str>>()
            .join(&self.delimiter);

        writeln!(self.output_file, "{}", joined).expect("Unable to write row to output");
        
        Ok(())
    }

    fn end(&mut self) -> Result<()> {
        self.output_file.flush().expect("Unable to flush output");
        
        Ok(())
    }
}
