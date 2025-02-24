use clap::Parser;
use std::cmp::min;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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
    pub file_name: Option<String>,

    /// Print headers
    #[clap(default_value_t = true, short, long)]
    pub with_headers: bool,
}

struct ColumnSpec {
    name: String,
    data_type: String,
    width: usize,
}

impl ColumnSpec {
    fn parse(spec: &str) -> ColumnSpec {
        let mut parts = spec.split(":");

        ColumnSpec {
            name: parts.next().unwrap().to_string(),
            data_type: parts.next().unwrap().to_string(),
            width: parts.next().unwrap().parse::<usize>().unwrap(), //TODO: handle the '*' case, which means open-ended
        }
    }

    fn parse_all(spec: &str) -> Vec<ColumnSpec> {
        spec.split(",").map(|x| ColumnSpec::parse(x)).collect()
    }

    fn format(&self, input: &str) -> String {
        match self.data_type.as_str() {
            "s" => format!("\"{}\"", input.to_string()),
            _ => input.to_string(),
        }
    }
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

        result.push(formatted_string.to_string());
    }

    result
}

// COLUMN1_NAME:str:10,COLUMN2_NAME:d:8,COLUMN3_NAME:f:12
fn main() {
    let args = Args::parse();
    let delimiter = args.delimiter.as_str();

    let spec = ColumnSpec::parse_all(args.spec.as_str());

    if args.with_headers {
        print_header(&spec);
    }

    let reader: Box<dyn BufRead> = match args.file_name {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(File::open(filename).unwrap())),
    };

    let iterator = reader.lines();

    for line in iterator {
        let line = line.unwrap();
        let parsed = parse_line(&line, &spec);
        let joined = parsed.join(delimiter);
        println!("{}", joined);
    }
}

fn print_header(spec: &Vec<ColumnSpec>) {
    let header_line = spec
        .iter()
        .map(|col| col.name.to_string())
        .collect::<Vec<String>>()
        .join(",");

    println!("{}", header_line);
}
