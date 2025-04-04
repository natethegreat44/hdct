pub trait RowWriter {
    fn begin(&mut self);
    fn write_row(&mut self, row: Vec<String>);
    fn end(&mut self);
}
