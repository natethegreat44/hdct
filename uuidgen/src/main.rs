use clap::Parser;
use hdct_helpers::clipboard_helper::paste;
use uuid::Uuid;

/// Create a UUID
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Include hyphens in the uuid. False by default.
    #[clap(default_value_t = false, short, long)]
    pub with_hyphens: bool,

    /// Convert the uuid to upper case
    #[clap(short, long)]
    pub upper: bool,

    /// Copy the resulting uuid to the clipboard
    #[clap(short, long)]
    pub copy_to_clipboard: bool,
}

fn main() {
    let args = Args::parse();

    let uuid = match args.with_hyphens {
        true => Uuid::new_v4().to_string(),
        _ => Uuid::new_v4().simple().to_string(),
    };

    let result = match args.upper {
        true => uuid.to_uppercase(),
        _ => uuid,
    };

    match args.copy_to_clipboard {
        true => paste(&result),
        _ => (),
    }

    println!("{result}");
}
