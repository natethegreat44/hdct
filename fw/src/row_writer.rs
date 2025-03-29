use crate::column_spec::ColumnSpec;

pub trait RowWriter {
    fn write_header(&self, spec: &Vec<ColumnSpec>);
    fn write_row(&self, items: Vec<String>);
}
