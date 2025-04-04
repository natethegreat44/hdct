use clap::Parser;
use hdct_helpers::io_helper::{reader_from_file_or_stdin, writer_to_file_or_stdout};
use std::cmp::min;
use std::io::BufRead;

mod column_spec;
mod delimited_output;
mod row_writer;
mod parquet_output;

use column_spec::ColumnSpec;
use delimited_output::DelimitedOutput;
// use parquet_output::ParquetOutput;
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

    /// File to read. If not specified, will expect stdin.
    #[clap(short='i', long="input-file")]
    pub input_file_name: Option<String>,

    /// File to read. If not specified, will expect stdin.
    #[clap(short='o', long="output-file")]
    pub output_file_name: Option<String>,

    #[clap(default_value="delimited", short='f', long="output-format")]
    pub output_file_format: String,

    /// Print headers
    #[clap(default_value_t = true, short, long)]
    pub with_headers: bool,
}

// COLUMN1_NAME:str:10,COLUMN2_NAME:d:8,COLUMN3_NAME:f:12
fn main() {
    let args = Args::parse();

    let spec = ColumnSpec::parse(args.spec.as_str());
    let reader = reader_from_file_or_stdin(args.input_file_name);
    let writer = writer_to_file_or_stdout(args.output_file_name).unwrap();

    let mut output: Box<dyn RowWriter> = match args.output_file_format.as_str() {
        "delimited" => Box::new(DelimitedOutput::new(args.delimiter.to_string(), &spec, writer)),
        // "parquet" => Box::new(ParquetOutput::new(&spec)),
        _ => unimplemented!(),
    };

    let iterator = reader.lines();

    output.begin();

    for line in iterator {
        let line = line.unwrap();
        let parsed = parse_line(&line, &spec);
        output.write_row(parsed);
    }

    output.end();
}

fn parse_line(line: &String, spec: &Vec<ColumnSpec>) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let line_length = line.chars().count();

    let mut pos: usize = 0;
    for column in spec {
        // TODO: If we've reached the end of the line, may be able to push empty strings for performance
        let end = min(line_length, pos + column.width);

        let substring = &line[pos..end];
        let formatted_string = column.format(substring);
        pos = min(line_length, column.width);

        result.push(formatted_string.trim().to_string());
    }

    result
}