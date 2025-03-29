pub trait RowWriter {
    fn write_header(&self);
    fn write_row(&self, row: Vec<String>);
}
