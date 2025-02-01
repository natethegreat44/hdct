use chrono::{DateTime, Utc};
use clap::Parser;
use hdct_helpers::clipboard_helper::paste;
use std::fmt::Write;

/// Convert from epoch time to human readable time. UTC only.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The input value to convert.
    input: i64,

    /// Copy the resulting uuid to the clipboard
    #[clap(short, long)]
    pub copy_to_clipboard: bool,
}

fn main() {
    let args = Args::parse();

    let utc_now = Utc::now();

    let dt: DateTime<Utc> =
        DateTime::<Utc>::from_timestamp(args.input, 0).expect("invalid timestamp");
    let diff = dt.signed_duration_since(utc_now);

    let mut result = String::new();
    writeln!(&mut result, "Epoch time: {}", args.input).unwrap();
    writeln!(&mut result, "UTC time: {}", dt).unwrap();

    let sign = if utc_now.timestamp_millis() - dt.timestamp_millis() > 0 {
        "-"
    } else {
        ""
    };

    writeln!(
        &mut result,
        "Relative to now: {sign}{:?}d {:02}h {:02}m {:02}.{:04}s",
        diff.num_days().abs(),
        diff.num_hours().abs(),
        diff.num_minutes().abs(),
        diff.num_seconds().abs() % 60,
        diff.num_milliseconds().abs() % 60_000
    )
    .unwrap();

    match args.copy_to_clipboard {
        true => paste(&result),
        _ => (),
    }

    println!("{}", result);
}
