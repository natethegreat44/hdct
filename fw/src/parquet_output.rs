use crate::row_writer::RowWriter;

pub struct ParquetOutput {

}

impl ParquetOutput {
    pub fn new() -> Self {
        Self {  }
    }
}

impl RowWriter for ParquetOutput {
    fn write_header(&self) {

    }

    fn write_row(&self, row: Vec<String>) {

    }
}

// https://github.com/apache/arrow-rs/blob/main/parquet/examples/write_parquet.rs