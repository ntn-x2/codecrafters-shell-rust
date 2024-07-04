use std::process::exit;

use crate::command::{Command, Context, TryWithArgsAndContext};

pub(crate) struct Exit {
    exit_code: i32,
}

impl TryWithArgsAndContext for Exit {
    fn try_with_args_and_context(args: &str, _context: Context) -> Result<Self, ()> {
        let exit_code = args.parse().map_err(|_| ())?;
        Ok(Self { exit_code })
    }
}

impl Command for Exit {
    fn execute(&self) -> Result<(), ()> {
        exit(self.exit_code)
    }
}
