pub mod epochtime_command;
pub mod uuid_command;

pub type CommandResult = Result<String, String>;

pub trait HdctCommand {
    fn run(self, verbosity: i32) -> CommandResult;
}
