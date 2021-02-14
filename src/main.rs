use clap::Clap;
use crate::clap_opts::{Opts, SubCommand};
use crate::commands::{HdctCommand, CommandResult};
use crate::helpers::clipboard_helper::paste;

mod clap_opts;
mod commands;
mod helpers;

fn main() {
    let opts: Opts = Opts::parse();

    match run(opts) {
        Ok(t) => {
            println!("Success!");
            println!("{}", t);
            std::process::exit(0)
        },
        Err(e) => {
            println!("Encountered an error: {}", e);
            std::process::exit(-1)
        }
    }
}

fn run(opts: Opts) -> CommandResult {
    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match opts.verbose {
        0 => (),
        x => println!("Verbosity level {}", x)
    }

    let result = match opts.subcmd {
        SubCommand::Uuid(u) => {
            u.run(opts.verbose)
        },
        SubCommand::EpochTime(e) => {
            e.run(opts.verbose)
        }
    };

    match &result {
        Ok(t) => {
            if opts.clipboard {
                println!("Pasting to clipboard.");
                paste(t.to_owned());
            }
        }
        _ => ()
    }

    result
}