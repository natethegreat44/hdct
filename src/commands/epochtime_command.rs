use clap::Clap;
use chrono::{NaiveDateTime, DateTime, Utc};
use crate::commands::clipboard_helper::paste;

/// Convert from epoch time to human readable time. UTC only.
#[derive(Clap)]
pub struct EpochTime {
    /// Place the resulting time on the clipboard
    #[clap(short)]
    pub clipboard: bool,

    /// The input value to convert.
    input: String,
}

impl EpochTime {
    pub fn run(self, verbosity: i32) {
        if verbosity > 0 {
            println!("Creating epoch time value from {:?}", self.input);
        }

        // TODO: Error checking
        let input_numeric: i64 = self.input.parse::<i64>().unwrap();

        let converted = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(input_numeric, 0), Utc);

        let result = format!("Epoch time is {:?}, UTC time is {:?}", input_numeric, converted);
        println!("{:?}", result);

        if self.clipboard {
            paste(result.to_owned());
            println!("Copied results to clipboard successfully.");
        }
    }
}