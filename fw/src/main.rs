use clap::Parser;
use hdct_helpers::io_helper::reader_from_file_or_stdin;
use std::cmp::min;
use std::fs::File;
use std::io::BufRead;
use indicatif::ProgressBar;

mod column_spec;
mod delimited_output;
mod parquet_output;
mod row_writer;

use column_spec::ColumnSpec;
use delimited_output::DelimitedOutput;
use parquet_output::ParquetOutput;
use row_writer::RowWriter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Fixed-width file spec
    #[clap(short = 's', long = "spec")]
    pub spec: String,

    /// Delimiter to use on the output data
    #[clap(default_value = ",", short, long)]
    pub delimiter: String,

    /// File to read. If not specified, will use stdin.
    #[clap(short = 'i', long = "input-file")]
    pub input_file_name: Option<String>,

    /// File to write.
    #[clap(short = 'o', long = "output-file")]
    pub output_file_name: String,

    /// Output format to write.
    #[clap(default_value = "delimited", short = 'f', long = "output-format")]
    pub output_file_format: String,

    /// Print headers
    #[clap(default_value_t = true, short, long)]
    pub with_headers: bool,

    /// Show progress
    #[clap(default_value_t = true, short, long)]
    pub progress: bool,
}

// COLUMN1_NAME:str:10,COLUMN2_NAME:d:8,COLUMN3_NAME:f:12
fn main() {
    let args = Args::parse();

    let specs = ColumnSpec::from_specs_str(args.spec.as_str());
    let (file_len, reader) = reader_from_file_or_stdin(args.input_file_name);
    let output_file = File::create(&args.output_file_name).unwrap(); //writer_to_file_or_stdout(args.output_file_name);

    let mut output: Box<dyn RowWriter> = match args.output_file_format.as_str() {
        "delimited" => Box::new(DelimitedOutput::new(
            args.delimiter.to_string(),
            &specs,
            output_file,
        )),
        "parquet" => Box::new(ParquetOutput::new(&specs, output_file)),
        _ => unimplemented!("Don't know how to handle that file format."),
    };

    let mut iterator = reader.lines();

    output.begin();

    let mut approx_line_count = 0u64;
    if args.progress {
        let first_line = iterator.next().unwrap().unwrap();
        let first_line_width = first_line.len();
        approx_line_count = file_len / first_line_width as u64;
        eprintln!("Approximate line count is {}", approx_line_count);
        let parsed = split_line_into_columns(&first_line, &specs);
        output.write_row(parsed);
    }

    let bar = ProgressBar::new(approx_line_count);
    for line in iterator {
        let line = line.unwrap();
        let parsed = split_line_into_columns(&line, &specs);
        output.write_row(parsed);
        bar.inc(1);
    }

    output.end();
    bar.finish();
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
