use std::{
    io::{stdout, Write},
    str::FromStr,
};

use crate::command::Command;

pub(crate) struct Echo {
    message: String,
}

impl FromStr for Echo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let message = {
            let split = s
                .split_once(' ')
                .map(|(first, second)| (first.trim(), second.trim()));
            let Some(("echo", second)) = split else {
                return Err(())
            };
            second
        };
        Ok(Self {
            message: message.to_string(),
        })
    }
}

impl Command for Echo {
    fn execute(&self) -> Result<(), String> {
        let mut stdout = stdout();
        stdout.write_all(self.message.as_bytes()).unwrap();
        stdout.write_all(b"\n").unwrap();
        stdout.flush().unwrap();
        Ok(())
    }
}
