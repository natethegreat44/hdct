use std::io::Result;

pub trait RowWriter {
    fn begin(&mut self) -> Result<()>;
    fn write_row(&mut self, row: Vec<String>) -> Result<()>;
    fn end(&mut self) -> Result<()>;
}
