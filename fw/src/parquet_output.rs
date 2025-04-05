use std::fs::File;
use arrow::datatypes::{DataType, Field, FieldRef, Schema};
use std::sync::Arc;
use arrow::array::StructArray;
use crate::RowWriter;
use crate::column_spec::ColumnSpec;

use parquet::arrow::ArrowWriter as ParquetWriter;
use parquet::basic::Encoding;
use parquet::file::properties::{BloomFilterPosition, WriterProperties};

pub struct ParquetOutput<'a> {
    specs: &'a Vec<ColumnSpec>,
    writer: ParquetWriter<File>,
    schema: Arc<Schema>,
    buffer: Vec<>
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
                    // "int32[]" => DataType::List(FieldRef::)
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
            schema.clone(),
            Some(properties)).unwrap();

        Self {
            specs,
            writer: parquet_writer,
            schema,
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

        // the writer expects batches, so this method should batch the records up
        // and then write when the size gets too big

        self.writer.write(&StructArray::new(
            self.schema.fields.clone(), // this seems super expensive
            row,
            None,
        ));
    }

    fn end(&mut self) {
        println!("end");
    }
}
// //
// // // https://github.com/apache/arrow-rs/blob/main/parquet/examples/write_parquet.rs
