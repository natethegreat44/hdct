use clap::Parser;
use uuid::Uuid;
use clipboard::{ClipboardContext, ClipboardProvider};

/// Create a UUID
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Simple format - no hyphens
    #[clap(short)]
    pub simple: bool,

    /// Convert the uuid to upper case
    #[clap(short)]
    pub upper: bool,

    /// Copy the resulting uuid to the clipboard
    #[clap(short)]
    pub copy_to_clipboard: bool,
}

fn paste(data: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(data.to_owned()).unwrap();
}

fn main() {
    let args = Args::parse();

    let uuid = match args.simple {
        true => Uuid::new_v4().simple().to_string(),
        _ => Uuid::new_v4().to_string(),
    };

    let result = match args.upper {
        true => uuid.to_uppercase(),
        _ => uuid,
    };

    match args.copy_to_clipboard {
        true => paste(&result),
        _ => ()
    }    

    println!("{result}");
    // Ok(result)
}