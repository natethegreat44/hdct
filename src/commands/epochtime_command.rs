use crate::commands::{CommandResult, HdctCommand};
use chrono::{DateTime, NaiveDateTime, Utc};
use clap::Clap;
use std::fmt::Write;

/// Convert from epoch time to human readable time. UTC only.
#[derive(Clap)]
pub struct EpochTime {
    /// The input value to convert.
    input: i64,
}

impl HdctCommand for EpochTime {
    fn run(self, verbosity: i32) -> CommandResult {
        if verbosity > 0 {
            println!("Creating epoch time value from {:?}", self.input);
        }

        let converted =
            DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.input, 0), Utc);

        let diff = converted.signed_duration_since(Utc::now());

        let mut result = String::new();
        writeln!(&mut result, "Epoch time: {}", self.input).unwrap();
        writeln!(&mut result, "UTC time: {}", converted).unwrap();
        writeln!(
            &mut result,
            "Relative to now: {:?}d {:02}h {:02}m {:02}.{:04}s",
            diff.num_days(),
            diff.num_hours(),
            diff.num_minutes(),
            diff.num_seconds(),
            diff.num_milliseconds()
        )
        .unwrap();

        Ok(result)
    }
}
