use clap::Parser;
use hdct_helpers::io_helper::reader_from_file_or_stdin;
use std::cmp::min;
use std::io::BufRead;

mod column_spec;
mod delimited_output;
mod row_writer;
mod parquet_output;

use column_spec::ColumnSpec;
use delimited_output::DelimitedOutput;
use row_writer::RowWriter;

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

        result.push(formatted_string.to_string());
    }

    result
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Fixed-width file spec
    #[clap(short, long)]
    pub spec: String,

    /// Delimiter to use on the output data
    #[clap(default_value = ",", short, long)]
    pub delimiter: String,

    /// File to read. If not specified, will expect stdin.
    #[clap(short, long)]
    pub input_file_name: Option<String>,

    /// Print headers
    #[clap(default_value_t = true, short, long)]
    pub with_headers: bool,
}

// COLUMN1_NAME:str:10,COLUMN2_NAME:d:8,COLUMN3_NAME:f:12
fn main() {
    let args = Args::parse();
    let delimiter = args.delimiter.as_str();
    let spec = ColumnSpec::parse(args.spec.as_str());
    let output = DelimitedOutput::new(delimiter.to_string(), &spec);

    if args.with_headers {
        output.write_header();
    }

    let reader = reader_from_file_or_stdin(args.input_file_name);

    let iterator = reader.lines();

    for line in iterator {
        let line = line.unwrap();
        let parsed = parse_line(&line, &spec);
        output.write_row(parsed);
    }
}
