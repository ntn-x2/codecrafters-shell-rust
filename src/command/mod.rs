use exit::Exit;

mod exit;

pub trait Command {
    fn execute(&self) -> Result<(), String>;
}

pub(crate) struct CommandParser;

impl CommandParser {
    pub(crate) fn parse(raw_command: &str) -> Result<Box<dyn Command>, String> {
        if let Ok(exit_command) = raw_command.parse::<Exit>() {
            Ok(Box::new(exit_command))
        } else {
            Err(format!("{}: command not found", raw_command.trim()))
        }
    }
}
