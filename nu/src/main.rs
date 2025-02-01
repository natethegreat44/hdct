use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Left margin size, used for printing line numbers
    #[clap(default_value_t = 5, short, long)]
    pub margin: usize,

    /// Maximum number of lines to read to infer header size
    #[clap(default_value_t = 10, short, long)]
    pub lines_to_read: usize,

    /// Don't show the numeric column-count header
    #[clap(default_value_t = false, short, long)]
    pub no_header: bool,

    /// File to read. If not specified, will expect stdin.Don't show the numeric column-count header
    #[clap(short, long)]
    pub file_name: Option<String>,
}

fn print_header(width: u32, margin_width: usize) {
    let loop_count = width.ilog10() + 1;

    for i in (1..loop_count + 1).rev() {
        print!("{: ^margin_width$}", "");
        let p10 = 10u32.pow(i - 1);
        for j in 0..width + 1 {
            if j % p10 == 0 {
                print!("{}", (j / p10) % 10);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn print_numbered_line(margin_width: usize, line_number: u32, line: &String) {
    println!("{:>margin_width$}: {}", line_number, line);
}

fn buffer_lines(count: usize, iterator: &mut Lines<Box<dyn BufRead>>) -> Vec<String> {
    let mut lines = Vec::new();
    for _ in 0..count {
        match iterator.next() {
            Some(Ok(line)) => lines.push(line),
            None => break,
            _ => {}
        }
    }

    lines
}

fn main() {
    let args = Args::parse();

    let input = args.file_name;
    let reader: Box<dyn BufRead> = match input {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(File::open(filename).unwrap())),
    };

    let mut iterator = reader.lines();

    let initial_lines = buffer_lines(args.lines_to_read, &mut iterator);
    let max_len = initial_lines.iter().map(|x| x.chars().count()).max().unwrap_or(0);

    if !args.no_header {
        print_header(max_len as u32, args.margin);
    }

    let mut line_no = 1;
    for line in initial_lines {
        if args.margin > 0 {
            print_numbered_line(args.margin - 1, line_no, &line);
        } else {
            println!("{}", line);
        }

        line_no += 1;
    }

    for line in iterator {
        if args.margin > 0 {
            print_numbered_line(args.margin - 1, line_no, &line.unwrap());
        } else {
            println!("{}", line.unwrap());
        }

        line_no += 1;
    }
}
