use std::cmp::max;
use std::io::{self, BufRead, Lines, StdinLock};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Left margin size, used for printing line numbers
    #[clap(default_value_t=5,short,long)]
    pub margin: usize,

    /// Number of lines to read to infer header size
    #[clap(default_value_t=10,short,long)]
    pub lines_to_read: usize,

    /// Don't show the numeric column-count header
    #[clap(default_value_t=false,short,long)]
    pub no_header: bool,
}

fn print_header(width: u32, margin_width: usize) {
    let loop_count = width.ilog10() + 1;

    for i in (1..loop_count+1).rev() {
        print!("{: ^margin_width$}", "");
        let p10 = 10u32.pow(i-1);
        for j in 0..width+1 {
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

fn buffer_lines(count: usize, iterator: &mut Lines<StdinLock>) -> Vec<String> {
    let mut lines = Vec::new();
    for _ in 0..count {
        // may need to carefully unwrap... panic is unlikely and ok for now
        lines.push(iterator.next().unwrap().unwrap());
    }

    lines
}

fn get_longest_line_size(lines: &Vec<String>) -> usize {
    let mut max_len = 0;
    for line in lines {
        max_len = max(max_len, line.chars().count());
    }
    max_len
}

fn main() {
    let args = Args::parse();

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let initial_lines= buffer_lines(args.lines_to_read, &mut iterator);
    let max_len = get_longest_line_size(&initial_lines);

    if !args.no_header {
        print_header(max_len as u32, args.margin);
    }

    let mut line_no = 1;
    for line in initial_lines {
        if args.margin > 0 {
            print_numbered_line(args.margin-1, line_no, &line);
        } else {
            println!("{}", line);
        }

        line_no += 1;
    }

    for line in iterator {
        if args.margin > 0 {
            print_numbered_line(args.margin-1, line_no, &line.unwrap());
        } else {
            println!("{}", line.unwrap());
        }

        line_no += 1;
    }
}
