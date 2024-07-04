pub trait Command {
    fn execute(&self) -> Result<(), String>;
}

pub(crate) struct CommandParser;

impl CommandParser {
    pub(crate) fn parse(raw_command: &str) -> Result<Box<dyn Command>, String> {
        Err(format!("{}: command not found", raw_command.trim()))
    }
}
