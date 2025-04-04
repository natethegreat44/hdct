// use std::io::Write;
// use std::sync::Arc;
// use arrow::array::{StructArray, UInt64Builder};
// use arrow::datatypes::DataType::UInt64;
// use arrow::datatypes::{DataType, Field, Schema};
//
// use crate::column_spec::ColumnSpec;
// use crate::RowWriter;
//
// pub struct ParquetOutput<'a> {
//     specs: &'a Vec<ColumnSpec>,
// }
//
// impl<'a> ParquetOutput<'a> {
//     pub fn new(specs: &'a Vec<ColumnSpec>) -> Self {
//         Self { specs }
//     }
// }
// //
// //     fn create_schema(specs: &'a Vec<ColumnSpec>) -> Arc<Schema> {
// //         let header_line = specs
// //             .iter()
// //             .map(|col|
// //                 Schema::new(vec![Field::new(
// //                     col.name.to_string(),
// //                     match col.data_type {
// //                         "str" -> DataType::Utf8
// //                     }
// //                     DataType::Utf8,
// //                     true)])
// //             .collect::<Vec<Schema>>();
// //         //let schema = Arc::new(Schema::new(vec![Field::new("id", UInt64, false)]));
// //
// //         schema
// //     }
// // }
// //
// impl RowWriter for ParquetOutput<'_> {
//     fn begin(&self) {
//
//     }
//
//     fn write_row(&self, row: Vec<String>) {
//
//     }
//
//     fn end(&self) {
//
//     }
//
//
// }
// //
// // // https://github.com/apache/arrow-rs/blob/main/parquet/examples/write_parquet.rs