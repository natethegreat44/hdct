use std::fs::File;
use arrow::datatypes::{DataType, Field, Schema};
use std::sync::Arc;

use crate::RowWriter;
use crate::column_spec::ColumnSpec;

use parquet::arrow::ArrowWriter as ParquetWriter;
use parquet::basic::Encoding;
use parquet::file::properties::{BloomFilterPosition, WriterProperties};

pub struct ParquetOutput<'a> {
    specs: &'a Vec<ColumnSpec>,
    writer: ParquetWriter<File>,
}

fn create_schema(specs: &Vec<ColumnSpec>) -> Arc<Schema> {
    // Field::new(name, datatype, nullable)
    let fields = specs
        .iter()
        .map(|col| {
            Field::new(
                col.name.to_string(),
                match col.data_type.as_str() {
                    "str" => DataType::Utf8,
                    "int64" => DataType::Int64,
                    "int32" => DataType::Int32,
                    "int16" => DataType::Int16,
                    "int8" => DataType::Int8,
                    "uint64" => DataType::UInt64,
                    "uint32" => DataType::UInt32,
                    "uint16" => DataType::UInt16,
                    "uint8" => DataType::UInt8,
                    _ => DataType::Utf8,
                },
                false,
            )
        })
        .collect::<Vec<Field>>();

    let schema = Arc::new(Schema::new(fields));

    schema
}

impl<'a> ParquetOutput<'a> {
    pub fn new(specs: &'a Vec<ColumnSpec>, output_file: File) -> Self {
        let schema = create_schema(&specs);

        let bloom_filter_position = BloomFilterPosition::AfterRowGroup;

        let properties = WriterProperties::builder()
            .set_column_bloom_filter_enabled("id".into(), true)
            .set_column_encoding("id".into(), Encoding::DELTA_BINARY_PACKED)
            .set_bloom_filter_position(bloom_filter_position)
            .build();

        let parquet_writer = ParquetWriter::try_new(
            output_file,
            schema,
            Some(properties)).unwrap();

        Self {
            specs,
            writer: parquet_writer,
        }
    }
}

// //
impl RowWriter for ParquetOutput<'_> {
    fn begin(&mut self) {
        println!("begin");
    }

    fn write_row(&mut self, row: Vec<String>) {
        print!(".");
    }

    fn end(&mut self) {
        println!("end");
    }
}
// //
// // // https://github.com/apache/arrow-rs/blob/main/parquet/examples/write_parquet.rs
