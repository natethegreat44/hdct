use std::cmp::min;
use clap::Parser;
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

    /// File to read. If not specified, will expect stdin.Don't show the numeric column-count header
    #[clap(short, long)]
    pub file_name: Option<String>,
}

fn parse_line(line: &String, spec: &Vec<usize>) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let line_length = line.chars().count();

    let mut pos: usize = 0;
    for slice in spec {
        // TODO: If we've reached the end of the line, may be able to push empty strings for performance
        let end = min(line_length, pos + slice);

        let substring = &line[pos..end];
        pos = min(line_length, *slice);

        result.push(substring.to_string());
    }

    result
}


// Header ideas:
// COLUMN1_NAME:10s,COLUMN2_NAME:8d,COLUMN3_NAME:8.4f
fn main() {
    let args = Args::parse();
    let delimiter = args.delimiter.as_str();
    let spec = args
        .spec
        .split(":")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

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
