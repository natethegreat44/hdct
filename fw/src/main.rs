use clap::{Parser};
use hdct_helpers::io_helper::{reader_from_file_or_stdin};
use std::cmp::min;
use std::fs::File;
use std::io::BufRead;

mod column_spec;
mod delimited_output;
mod row_writer;
mod parquet_output;

use column_spec::ColumnSpec;
use delimited_output::DelimitedOutput;
use parquet_output::ParquetOutput;
use row_writer::RowWriter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Fixed-width file spec
    #[clap(short='s', long="spec")]
    pub spec: String,

    /// Delimiter to use on the output data
    #[clap(default_value = ",", short, long)]
    pub delimiter: String,

    /// File to read. If not specified, will use stdin.
    #[clap(short='i', long="input-file")]
    pub input_file_name: Option<String>,

    /// File to write.
    #[clap(short='o', long="output-file")]
    pub output_file_name: String,

    /// Output format to write.
    #[clap(default_value="delimited", short='f', long="output-format")]
    pub output_file_format: String,

    /// Print headers
    #[clap(default_value_t = true, short, long)]
    pub with_headers: bool,
}

// COLUMN1_NAME:str:10,COLUMN2_NAME:d:8,COLUMN3_NAME:f:12
fn main() {
    let args = Args::parse();

    let specs = ColumnSpec::from_specs(args.spec.as_str());
    let reader = reader_from_file_or_stdin(args.input_file_name);
    let output_file = File::create(args.output_file_name).unwrap(); //writer_to_file_or_stdout(args.output_file_name);

    let mut output: Box<dyn RowWriter> = match args.output_file_format.as_str() {
        "delimited" => Box::new(DelimitedOutput::new(args.delimiter.to_string(), &specs, output_file)),
        "parquet" =>  Box::new(ParquetOutput::new(&specs, output_file)),
        _ => unimplemented!(),
    };

    let iterator = reader.lines();

    output.begin();

    for line in iterator {
        let line = line.unwrap();
        let parsed = split_line_into_columns(&line, &specs);
        output.write_row(parsed);
    }

    output.end();
}

fn split_line_into_columns(line: &String, spec: &Vec<ColumnSpec>) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let line_length = line.chars().count();

    let mut pos: usize = 0;
    for column in spec {
        let end = min(line_length, pos + column.width);

        let substring = &line[pos..end];
        let parsed = column.parse(substring);
        pos = min(line_length, pos + column.width);

        result.push(parsed.to_string());
    }

    result
}