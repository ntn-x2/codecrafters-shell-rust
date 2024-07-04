use std::io::BufRead;
#[allow(unused_imports)]
use std::io::Write;

use command::{Command, CommandParser};

mod command;

pub fn parse_command<Input: BufRead>(input: &mut Input) -> Result<Box<dyn Command>, String> {
    {
        let mut buffer = String::new();
        input.read_line(&mut buffer).unwrap();
        CommandParser::parse(buffer.as_str())
    }
}
