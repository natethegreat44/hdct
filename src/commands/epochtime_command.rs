use clap::Clap;
use chrono::{NaiveDateTime, DateTime, Utc};
use crate::commands::{HdctCommand, CommandResult};

/// Convert from epoch time to human readable time. UTC only.
#[derive(Clap)]
pub struct EpochTime {
    /// The input value to convert.
    input: String,
}

impl HdctCommand for EpochTime {
    fn run(self, verbosity: i32) -> CommandResult {
        if verbosity > 0 {
            println!("Creating epoch time value from {:?}", self.input);
        }

        let parsed = self.input.parse::<i64>();
        if parsed.is_err() {
            return Err(format!("Error parsing {:?}; expected a numeric value.", self.input));
        }

        let parsed = parsed.unwrap(); // Not sure I like reusing the variable, but it's proper rust

        let converted = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(parsed, 0), Utc);

        let result = format!("Epoch time is {:?}, UTC time is {:?}", parsed, converted);

        //
        // if self.clipboard {
        //     paste(result);
        //     println!("Copied results to clipboard successfully.");
        // }

        Ok(result)
    }
}
