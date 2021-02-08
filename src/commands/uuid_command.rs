use clap::Clap;
use uuid::Uuid as UuidGen;
use crate::commands::clipboard_helper::paste;

/// Create a UUID
#[derive(Clap)]
pub struct Uuid {
    /// Place the resulting UUID on the clipboard
    #[clap(short)]
    pub clipboard: bool,

    /// Format of the resulting uuids - with or without hyphens
    #[clap(short)]
    pub hyphens: bool,

    /// Convert the uuid to upper case
    #[clap(short)]
    pub upper: bool,
}

impl Uuid {
    pub fn run(self, verbosity: i32) {
        if verbosity > 0 {
            println!("Creating uuid with format {:?} and clipboard {:?}", self.hyphens, self.clipboard);
        }

        let my_uuid = match self.hyphens {
            true => UuidGen::new_v4().to_string(),
            _ => UuidGen::new_v4().to_simple().to_string(),
        };

        let result = match self.upper {
            true => my_uuid.to_uppercase(),
            _ => my_uuid
        };

        println!("{}", result);

        if self.clipboard {
            paste(result.to_owned());
            println!("Copied guid {:?} to clipboard successfully.", result);
        }
    }
}