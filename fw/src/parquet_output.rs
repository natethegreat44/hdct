use crate::RowWriter;
use crate::column_spec::ColumnSpec;
use anyhow::{Context, Result, anyhow, bail};
use arrow::array::{
    ArrayBuilder, BooleanBuilder, Float32Builder, Float64Builder, Int8Builder, Int16Builder,
    Int32Builder, Int64Builder, ListBuilder, RecordBatch, StringBuilder, UInt8Builder,
    UInt16Builder, UInt32Builder, UInt64Builder,
};
use arrow::datatypes::{DataType, Field, Schema, SchemaRef};
use parquet::arrow::ArrowWriter;
use parquet::basic::{Compression, Encoding};
use parquet::file::properties::{BloomFilterPosition, WriterProperties};
use std::fs::File;
use std::sync::Arc;

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
            .set_compression(Compression::SNAPPY)
            .build();

        let builders = Self::create_builders(&schema);

        let parquet_writer = ArrowWriter::try_new(output_file, schema.clone(), Some(properties))
            .expect("Error creating parquet writer");

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
                        "int8" => DataType::Int8,
                        "int16" => DataType::Int16,
                        "int32" => DataType::Int32,
                        "int64" => DataType::Int64,
                        "uint8" => DataType::UInt8,
                        "uint16" => DataType::UInt16,
                        "uint32" => DataType::UInt32,
                        "uint64" => DataType::UInt64,
                        "f32" => DataType::Float32,
                        "f64" => DataType::Float64,
                        "int32[]" => {
                            DataType::List(Arc::new(Field::new("item", DataType::UInt32, true)))
                        }
                        _ => panic!("Don't know how to handle data type {}", col.data_type), //DataType::Utf8, //Probably should bail here
                    },
                    true,
                )
            })
            .collect::<Vec<Field>>();

        let schema = Arc::new(Schema::new(fields));

        schema
    }

    /// Creates Arrow ArrayBuilders based on the schema.
    fn create_builders(schema: &SchemaRef) -> Vec<Box<dyn ArrayBuilder>> {
        schema
            .fields()
            .iter()
            .map(|field| match field.data_type() {
                DataType::Int8 => Box::new(Int8Builder::new()) as Box<dyn ArrayBuilder>,
                DataType::Int16 => Box::new(Int16Builder::new()) as Box<dyn ArrayBuilder>,
                DataType::Int32 => Box::new(Int32Builder::new()) as Box<dyn ArrayBuilder>,
                DataType::Int64 => Box::new(Int64Builder::new()) as Box<dyn ArrayBuilder>,
                DataType::UInt8 => Box::new(UInt8Builder::new()) as Box<dyn ArrayBuilder>,
                DataType::UInt16 => Box::new(UInt16Builder::new()) as Box<dyn ArrayBuilder>,
                DataType::UInt32 => Box::new(UInt32Builder::new()) as Box<dyn ArrayBuilder>,
                DataType::UInt64 => Box::new(UInt64Builder::new()) as Box<dyn ArrayBuilder>,
                DataType::List(field_ref) if *field_ref.data_type() == DataType::UInt32 => {
                    Box::new(ListBuilder::new(UInt32Builder::new())) as Box<dyn ArrayBuilder>
                }
                DataType::Float32 => Box::new(Float32Builder::new()) as Box<dyn ArrayBuilder>,
                DataType::Float64 => Box::new(Float64Builder::new()) as Box<dyn ArrayBuilder>,
                DataType::Boolean => Box::new(BooleanBuilder::new()) as Box<dyn ArrayBuilder>,
                DataType::Utf8 => Box::new(StringBuilder::new()) as Box<dyn ArrayBuilder>,
                // Add builders for other supported types here
                dt => panic!("Unsupported data type for builder creation: {:?}", dt), // Should not happen if schema parsing is correct
            })
            .collect()
    }
}

//
impl RowWriter for ParquetOutput {
    fn begin(&mut self) {
        // Nothing to do here
    }

    fn write_row(&mut self, row: Vec<String>) {
        for (pos, col) in row.iter().enumerate() {
            let val = col.trim();
            let builder = &mut self.builders[pos];
            let data_type = self.schema.fields[pos].data_type();
            append_value(builder, data_type, val)
                .expect(format!("Unable to append value {}", val).as_str());
        }

        self.record_count += 1;

        if self.record_count % 1000 == 0 {
            //TODO: Could change the trait to return a Result<()>
            write_batch(&mut self.writer, self.schema.clone(), &mut self.builders)
                .expect("failed to write row");
        }
    }

    fn end(&mut self) {
        write_batch(&mut self.writer, self.schema.clone(), &mut self.builders)
            .expect("Unable to write last batch of records");

        self.writer.flush().expect("Error flushing writer");
        self.writer
            .finish()
            .expect("Unable to finish parquet writer");
    }
}

fn write_batch(
    writer: &mut ArrowWriter<File>,
    schema: SchemaRef,
    builders: &mut [Box<dyn ArrayBuilder>],
) -> Result<()> {
    let columns = builders
        .iter_mut()
        .map(|builder| builder.finish()) // This consumes the builder's data
        .collect::<Vec<_>>();

    match RecordBatch::try_new(schema.clone(), columns) {
        Ok(batch) => {
            if batch.num_rows() > 0 {
                // Only write if there's data
                writer
                    .write(&batch)
                    .context("Failed to write RecordBatch to Parquet writer")?;
            } else {
                eprintln!("No rows to write!");
            }
        }
        Err(e) => bail!("Error creating RecordBatch writer: {}", e),
    }

    Ok(())
}

/// Macro to handle downcasting, empty check, parsing, and appending for numeric types.
macro_rules! builder {
    // Use a block to scope the downcast result and return a Result
    ($builder:expr, $builder_type:ty, $type_name:expr) => {{
        $builder
            .as_any_mut()
            .downcast_mut::<$builder_type>()
            .ok_or_else(|| anyhow!("Builder type mismatch for {}", $type_name))?
    }};
}

macro_rules! numeric_append {
    // $builder: The mutable Box<dyn ArrayBuilder> expression
    // $value_str: The input string slice expression
    // $builder_type: The concrete builder type (e.g., Int32Builder)
    // $parse_type: The Rust type to parse into (e.g., i32)
    // $type_name: A string literal representing the type name for error messages (e.g., "Int32")
    ($builder:expr, $value_str:expr, $builder_type:ty, $parse_type:ty, $type_name:expr) => {{
        let concrete_builder = builder!($builder, $builder_type, $type_name);
        if $value_str.is_empty() {
            concrete_builder.append_null();
        } else {
            match $value_str.parse::<$parse_type>() {
                Ok(val) => concrete_builder.append_value(val),
                Err(e) => {
                    bail!("Unable to parse '{}' as {}: {}", $value_str, $type_name, e);
                }
            }
        }
    }}; // The block evaluates to Result<(), anyhow::Error>};};
}

macro_rules! numeric_list_append {
    // $builder: The mutable Box<dyn ArrayBuilder> expression
    // $value_str: The input string slice expression
    // $builder_type: The concrete builder type (e.g., Int32Builder)
    // $parse_type: The Rust type to parse into (e.g., i32)
    // $type_name: A string literal representing the type name for error messages (e.g., "Int32")
    ($builder:expr, $value_str:expr, $builder_type:ty, $parse_type:ty, $type_name:expr) => {{
        let concrete_builder = builder!($builder, $builder_type, $type_name);

        if $value_str.is_empty() {
            concrete_builder.append_null();
        } else {
            let values_builder = concrete_builder.values();
            for (i, c) in $value_str.trim().chars().enumerate() {
                // Try parsing the element as i32
                match c.to_digit(10) {
                    Some(val) => values_builder.append_value(val),
                    None => {
                        bail!(
                            "Unable to parse element '{}' at position {} as {}",
                            $value_str,
                            i,
                            $type_name
                        );
                    }
                }
            }

            // Crucial: Mark the end of the current list; signifies it's a valid (non-null) list
            concrete_builder.append(true);
        }
    }}; // The block evaluates to Result<(), anyhow::Error>};};
}

fn append_value(
    builder: &mut Box<dyn ArrayBuilder>,
    data_type: &DataType,
    value_str: &str,
) -> Result<()> {
    match data_type {
        DataType::Int8 => numeric_append!(builder, value_str, Int8Builder, i8, "Int8"),
        DataType::Int16 => numeric_append!(builder, value_str, Int16Builder, i16, "Int16"),
        DataType::Int32 => numeric_append!(builder, value_str, Int32Builder, i32, "Int32"),
        DataType::Int64 => numeric_append!(builder, value_str, Int64Builder, i64, "Int64"),
        DataType::UInt8 => numeric_append!(builder, value_str, UInt8Builder, u8, "UInt8"),
        DataType::UInt16 => numeric_append!(builder, value_str, UInt16Builder, u16, "UInt16"),
        DataType::UInt32 => numeric_append!(builder, value_str, UInt32Builder, u32, "UInt32"),
        DataType::UInt64 => numeric_append!(builder, value_str, UInt64Builder, u64, "UInt64"),
        DataType::List(field_ref) if *field_ref.data_type() == DataType::UInt32 => {
            numeric_list_append!(
                builder,
                value_str,
                ListBuilder<UInt32Builder>,
                u32,
                "List<UInt32>"
            )
        }
        DataType::Float32 => numeric_append!(builder, value_str, Float32Builder, f32, "Float32"),
        DataType::Float64 => numeric_append!(builder, value_str, Float64Builder, f64, "Float64"),
        DataType::Boolean => {
            let concrete_builder = builder!(builder, BooleanBuilder, "Boolean");
            match value_str.to_lowercase().as_str() {
                "true" | "t" | "1" | "yes" | "y" => concrete_builder.append_value(true),
                "false" | "f" | "0" | "no" | "n" => concrete_builder.append_value(false),
                "" => concrete_builder.append_null(), // Empty string as null boolean explicitly
                _ => bail!(
                    "Cannot parse '{}' as Boolean. Use true/false/1/0 etc.",
                    value_str
                ),
            }
        }
        DataType::Utf8 => {
            let concrete_builder = builder!(builder, StringBuilder, "String");
            concrete_builder.append_value(value_str);
        }
        // Add parsing logic for other supported types here
        dt => bail!("Unsupported data type for value appending: {:?}", dt),
    }

    Ok(())
}

// https://github.com/apache/arrow-rs/blob/main/parquet/examples/write_parquet.rs
