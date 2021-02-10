use clap::Clap;
use chrono::{NaiveDateTime, DateTime, Utc};
use crate::commands::clipboard_helper::paste;
use crate::commands::HdctCommand;

/// Convert from epoch time to human readable time. UTC only.
#[derive(Clap)]
pub struct EpochTime {
    /// Place the resulting time on the clipboard
    #[clap(short)]
    pub clipboard: bool,

    /// The input value to convert.
    input: String,
}

impl HdctCommand for EpochTime {
    fn run(self, verbosity: i32) -> i32 {
        if verbosity > 0 {
            println!("Creating epoch time value from {:?}", self.input);
        }

        let parsed = self.input.parse::<i64>();
        if parsed.is_err() {
            println!("Error parsing {:?}; expected a numeric value.", self.input);
            return 1;
        }

        let parsed = parsed.unwrap(); // Not sure I like reusing the variable, but it's proper rust

        let converted = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(parsed, 0), Utc);

        let result = format!("Epoch time is {:?}, UTC time is {:?}", parsed, converted);
        println!("{:?}", result);

        if self.clipboard {
            paste(result.to_owned());
            println!("Copied results to clipboard successfully.");
        }

        0
    }
}
