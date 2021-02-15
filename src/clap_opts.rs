use crate::commands::epochtime_command::EpochTime;
use crate::commands::uuid_command::Uuid;
use clap::Clap;

/// A suite (in progress!) of handy developer command line tools.
#[derive(Clap)]
#[clap(version = "1.0", author = "Nathan Blair <nathan.blair@gmail.com>")]
pub struct Opts {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: i32,

    /// Place the result of the command on the clipboard
    #[clap(short)]
    pub clipboard: bool,

    /// The required subcommand
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    Uuid(Uuid),
    EpochTime(EpochTime),
}
