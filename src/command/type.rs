use std::{
    collections::BTreeSet,
    io::{stdout, Write},
};

use crate::command::{Command, Context, TryWithArgsAndContext};

pub(crate) struct Type {
    command_string: String,
    recognized_commands: BTreeSet<String>,
}

impl TryWithArgsAndContext for Type {
    fn try_with_args_and_context(args: &str, context: Context) -> Result<Self, ()> {
        let command_string = args.to_string();
        Ok(Self {
            command_string,
            recognized_commands: context.recognized_commands,
        })
    }
}

impl Command for Type {
    fn execute(&self) -> Result<(), ()> {
        let mut stdout = stdout();
        if self.recognized_commands.contains(&self.command_string) {
            stdout
                .write_all(format!("{} is a shell builtin", self.command_string).as_bytes())
                .unwrap();
        } else {
            stdout
                .write_all(format!("{}: not found", self.command_string).as_bytes())
                .unwrap();
        }
        stdout.write_all(b"\n").unwrap();
        stdout.flush().unwrap();
        Ok(())
    }
}
