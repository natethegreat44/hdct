pub mod uuid_command;
pub mod epochtime_command;
pub mod clipboard_helper;

pub trait HdctCommand {
    fn run(self, verbosity: i32) -> i32;
}