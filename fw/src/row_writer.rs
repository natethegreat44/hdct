pub trait RowWriter {
    fn begin(&self);
    fn write_row(&self, row: Vec<String>);
    fn end(&self);
}
