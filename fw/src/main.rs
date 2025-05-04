use clap::Parser;
use hdct_helpers::io_helper::reader_from_file_or_stdin;
use indicatif::ProgressBar;
use std::cmp::min;
use std::fs::File;
use std::io::BufRead;

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
    #[clap(default_value_t = false, short, long)]
    pub no_progress: bool,
}

// COLUMN1_NAME:str:10,COLUMN2_NAME:d:8,COLUMN3_NAME:f:12
fn main() {
    let args = Args::parse();

    let specs = ColumnSpec::from_specs_str(args.spec.as_str());
    let (file_len, reader) = reader_from_file_or_stdin(args.input_file_name);
    let output_file = File::create(&args.output_file_name).expect("Unable to create output file"); //writer_to_file_or_stdout(args.output_file_name);

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

    output.begin().expect("Unable to start output");

    let mut approx_line_count = 0u64;
    if !args.no_progress {
        let first_line = iterator
            .next()
            .expect("Unable to iterate")
            .expect("Unable to read first line");
        let first_line_width = first_line.len();
        approx_line_count = file_len / first_line_width as u64;
        eprintln!("Approximate line count is {}", approx_line_count);
        let parsed = split_line_into_columns(&first_line, &specs);
        output.write_row(parsed).expect("Unable to write row");
    }

    let bar = if args.no_progress {
        ProgressBar::hidden()
    } else {
        ProgressBar::new(approx_line_count)
    };
    
    iterator.for_each(|line| {
        output.write_row(split_line_into_columns(&line.unwrap(), &specs)).expect("Unable to write row");
        bar.inc(1);
    });
    bar.finish();

    output.end().expect("Unable to end output");
}

fn split_line_into_columns(line: &String, spec: &Vec<ColumnSpec>) -> Vec<String> {
    let line_length = line.chars().count();
    let mut pos: usize = 0;
    
    spec.iter().map(|column| {
        let end = min(line_length, pos + column.width);

        let substring = &line[pos..end];
        let parsed = column.parse(substring);
        pos = min(line_length, pos + column.width);

        parsed.to_string()
    }).collect()
}
