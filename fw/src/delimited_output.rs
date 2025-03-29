use crate::column_spec::ColumnSpec;
use crate::row_writer::RowWriter;

pub struct DelimitedOutput {
    pub delimiter: String,
}

impl DelimitedOutput {
    pub fn new(delimiter: String) -> Self {
        Self { delimiter }
    }
}

impl RowWriter for DelimitedOutput {
    fn write_header(&self, spec: &Vec<ColumnSpec>) {
        let header_line = spec
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
}
