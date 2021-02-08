use clap::Clap;
use crate::clap_opts::Opts;
use crate::clap_opts::SubCommand;

mod clap_opts;
mod commands;

fn main() {
    let opts: Opts = Opts::parse();

    run(opts);
}

fn run(opts: Opts) {
    // Gets a value for config if supplied by user, or defaults to "default.conf"
    // println!("Value for config: {}", opts.config);
    // println!("Using input file: {}", opts.input);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    // match opts.verbose {
    //     0 => println!("No verbose info"),
    //     1 => println!("Some verbose info"),
    //     2 => println!("Tons of verbose info"),
    //     3 | _ => println!("Don't be crazy"),
    // }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::Test(t) => {
            if t.debug {
                println!("Printing debug info...");
            } else {
                println!("Printing normally...");
            }
        },
        SubCommand::Uuid(u) => {
            u.run(opts.verbose);
        },
        SubCommand::EpochTime(e) => {
            e.run(opts.verbose);
        }
    }
}