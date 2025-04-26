use crate::RowWriter;
use crate::column_spec::ColumnSpec;
use anyhow::{Context, Result, anyhow, bail};
use arrow::array::{
    ArrayBuilder, BooleanBuilder, Float32Builder, Float64Builder, Int32Builder, Int64Builder,
    RecordBatch, StringBuilder,
};
use arrow::datatypes::{DataType, Field, Schema, SchemaRef};
use std::error::Error;
use std::fs::File;
use std::sync::Arc;

use parquet::arrow::ArrowWriter;
use parquet::basic::Encoding;
use parquet::file::properties::{BloomFilterPosition, WriterProperties};

pub struct ParquetOutput {
    writer: ArrowWriter<File>,
    schema: SchemaRef,
    builders: Vec<Box<dyn ArrayBuilder>>,
    record_count: u32,
}

impl ParquetOutput {
    pub fn new(specs: &Vec<ColumnSpec>, output_file: File) -> Self {
        let schema = Self::create_schema(&specs);

        let bloom_filter_position = BloomFilterPosition::AfterRowGroup;

        let properties = WriterProperties::builder()
            .set_column_bloom_filter_enabled("id".into(), true)
            .set_column_encoding("id".into(), Encoding::DELTA_BINARY_PACKED)
            .set_bloom_filter_position(bloom_filter_position)
            .build();

        let builders = Self::create_builders(&schema, 1000);

        let parquet_writer =
            ArrowWriter::try_new(output_file, schema.clone(), Some(properties)).unwrap();

        Self {
            writer: parquet_writer,
            schema,
            builders,
            record_count: 0,
        }
    }

    fn create_schema(specs: &Vec<ColumnSpec>) -> Arc<Schema> {
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
                    true,
                )
            })
            .collect::<Vec<Field>>();

        let schema = Arc::new(Schema::new(fields));

        schema
    }

    /// Creates Arrow ArrayBuilders based on the schema.
    fn create_builders(schema: &SchemaRef, capacity: usize) -> Vec<Box<dyn ArrayBuilder>> {
        schema
            .fields()
            .iter()
            .map(|field| match field.data_type() {
                DataType::Int32 => {
                    Box::new(Int32Builder::with_capacity(capacity)) as Box<dyn ArrayBuilder>
                }
                DataType::Int64 => {
                    Box::new(Int64Builder::with_capacity(capacity)) as Box<dyn ArrayBuilder>
                }
                DataType::Float32 => {
                    Box::new(Float32Builder::with_capacity(capacity)) as Box<dyn ArrayBuilder>
                }
                DataType::Float64 => {
                    Box::new(Float64Builder::with_capacity(capacity)) as Box<dyn ArrayBuilder>
                }
                DataType::Boolean => {
                    Box::new(BooleanBuilder::with_capacity(capacity)) as Box<dyn ArrayBuilder>
                }
                DataType::Utf8 => Box::new(StringBuilder::with_capacity(capacity, capacity * 10))
                    as Box<dyn ArrayBuilder>, // Estimate string data size
                // Add builders for other supported types here
                dt => panic!("Unsupported data type for builder creation: {:?}", dt), // Should not happen if schema parsing is correct
            })
            .collect()
    }
}

// //
impl RowWriter for ParquetOutput {
    fn begin(&mut self) {
        println!("begin");
    }

    fn write_row(&mut self, row: Vec<String>) {
        print!(".");

        for (pos, col) in row.iter().enumerate() {
            let val = col.trim();
            let builder = &mut self.builders[pos];
            let data_type = self.schema.fields[pos].data_type();
            _ = append_value(builder, data_type, val)
        }

        self.record_count += 1;

        if self.record_count % 1000 == 0 {
            _ = write_batch(&mut self.writer, self.schema.clone(), &mut self.builders);
        }
    }

    fn end(&mut self) {
        if self.record_count % 1000 != 0 {
            _ = write_batch(&mut self.writer, self.schema.clone(), &mut self.builders);
        }

        self.writer.finish().unwrap();
    }
}

fn write_batch(
    writer: &mut ArrowWriter<File>,
    schema: SchemaRef,
    builders: &mut [Box<dyn ArrayBuilder>],
) -> Result<()> {
    // Finish builders and create arrays
    let columns = builders
        .iter_mut()
        .map(|builder| builder.finish()) // This consumes the builder's data
        .collect::<Vec<_>>();

    // Create RecordBatch
    let batch = RecordBatch::try_new(schema.clone(), columns);

    match batch {
        Ok(batch) => {
            // Write batch to Parquet
            if batch.num_rows() > 0 {
                // Only write if there's data
                writer
                    .write(&batch)
                    .context("Failed to write RecordBatch to Parquet writer")?;
            }
        }
        Err(e) => return Err(anyhow!("Error creating RecordBatch writer: {}", e)),
    }

    Ok(())
}

fn append_value(
    builder: &mut Box<dyn ArrayBuilder>,
    data_type: &DataType,
    value_str: &str,
) -> Result<()> {
    // Centralized error creation
    let create_parse_error = |data: String, data_type: &str, e: Box<dyn Error>| -> anyhow::Error {
        anyhow!(format!("Cannot parse '{}' as {}: {}", data, data_type, e))
    };

    match data_type {
        DataType::Int32 => {
            let concrete_builder = builder
                .as_any_mut()
                .downcast_mut::<Int32Builder>()
                .ok_or_else(|| anyhow!("Builder type mismatch for Int32"))?;
            if value_str.is_empty() {
                concrete_builder.append_null();
            } else {
                match value_str.parse::<i32>() {
                    Ok(val) => concrete_builder.append_value(val),
                    Err(e) => {
                        return Err(create_parse_error(
                            value_str.to_string(),
                            "Int32",
                            Box::new(e),
                        ));
                    }
                }
            }
        }
        DataType::Int64 => {
            let concrete_builder = builder
                .as_any_mut()
                .downcast_mut::<Int64Builder>()
                .ok_or_else(|| anyhow!("Builder type mismatch for Int64"))?;
            if value_str.is_empty() {
                concrete_builder.append_null();
            } else {
                match value_str.parse::<i64>() {
                    Ok(val) => concrete_builder.append_value(val),
                    Err(e) => {
                        return Err(create_parse_error(
                            value_str.to_string(),
                            "Int64",
                            Box::new(e),
                        ));
                    }
                }
            }
        }
        DataType::Float32 => {
            let concrete_builder = builder
                .as_any_mut()
                .downcast_mut::<Float32Builder>()
                .ok_or_else(|| anyhow!("Builder type mismatch for Float32"))?;
            if value_str.is_empty() {
                concrete_builder.append_null();
            } else {
                match value_str.parse::<f32>() {
                    Ok(val) => concrete_builder.append_value(val),
                    Err(e) => {
                        return Err(create_parse_error(
                            value_str.to_string(),
                            "Float32",
                            Box::new(e),
                        ));
                    }
                }
            }
        }
        DataType::Float64 => {
            let concrete_builder = builder
                .as_any_mut()
                .downcast_mut::<Float64Builder>()
                .ok_or_else(|| anyhow!("Builder type mismatch for Float64"))?;
            if value_str.is_empty() {
                concrete_builder.append_null();
            } else {
                match value_str.parse::<f64>() {
                    Ok(val) => concrete_builder.append_value(val),
                    Err(e) => {
                        return Err(create_parse_error(
                            value_str.to_string(),
                            "Float64",
                            Box::new(e),
                        ));
                    }
                }
            }
        }
        DataType::Boolean => {
            let concrete_builder = builder
                .as_any_mut()
                .downcast_mut::<BooleanBuilder>()
                .ok_or_else(|| anyhow!("Builder type mismatch for Boolean"))?;
            match value_str.to_lowercase().as_str() {
                "true" | "t" | "1" | "yes" | "y" => concrete_builder.append_value(true),
                "false" | "f" | "0" | "no" | "n" => concrete_builder.append_value(false),
                "" => concrete_builder.append_null(), // Empty string as null boolean explicitly
                _ => bail!(format!(
                    "Cannot parse '{}' as Boolean. Use true/false/1/0 etc.",
                    value_str
                )),
            }
        }
        // String treats empty strings as "", not NULL by default with append_value.
        // To make empty strings NULL, change append_value("") to append_null().
        DataType::Utf8 => {
            let concrete_builder = builder
                .as_any_mut()
                .downcast_mut::<StringBuilder>()
                .ok_or_else(|| anyhow!("Builder type mismatch for String"))?;
            // Decide how to handle empty strings for Utf8:
            // Option 1: Treat empty string as NULL
            // if value_str.is_empty() {
            //     concrete_builder.append_null();
            // } else {
            //     concrete_builder.append_value(value_str);
            // }
            // Option 2: Treat empty string as "" (current behavior)
            concrete_builder.append_value(value_str);
        }
        // Add parsing logic for other supported types here
        dt => bail!("Unsupported data type for value appending: {:?}", dt),
    }

    Ok(())
}

// https://github.com/apache/arrow-rs/blob/main/parquet/examples/write_parquet.rs
