use clap::Clap;

/// A simple test command
#[derive(Clap)]
pub struct Test {
    /// Print debug info
    #[clap(short)]
    pub debug: bool
}