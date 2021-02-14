use clap::Clap;
use uuid::Uuid as UuidGen;
use crate::commands::{HdctCommand, CommandResult};

/// Create a UUID
#[derive(Clap)]
pub struct Uuid {
    /// Format of the resulting uuids - with or without hyphens
    #[clap(short)]
    pub hyphens: bool,

    /// Convert the uuid to upper case
    #[clap(short)]
    pub upper: bool,
}

impl HdctCommand for Uuid {
    fn run(self, verbosity: i32) -> CommandResult {
        if verbosity > 0 {
            println!("Creating UUID");
        }

        let my_uuid = match self.hyphens {
            true => UuidGen::new_v4().to_string(),
            _ => UuidGen::new_v4().to_simple().to_string(),
        };

        let result = match self.upper {
            true => my_uuid.to_uppercase(),
            _ => my_uuid
        };

        Ok(result)
    }
}