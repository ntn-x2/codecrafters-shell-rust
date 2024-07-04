use std::io::{stdout, Write};

use crate::command::{Command, Context, TryWithArgsAndContext};

pub(crate) struct Echo {
    message: String,
}

impl TryWithArgsAndContext for Echo {
    fn try_with_args_and_context(args: &str, _context: Context) -> Result<Self, ()> {
        Ok(Self {
            message: args.to_string(),
        })
    }
}

impl Command for Echo {
    fn execute(&self) -> Result<(), ()> {
        let mut stdout = stdout();
        stdout.write_all(self.message.as_bytes()).unwrap();
        stdout.write_all(b"\n").unwrap();
        stdout.flush().unwrap();
        Ok(())
    }
}
