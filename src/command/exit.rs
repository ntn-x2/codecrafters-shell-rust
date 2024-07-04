use std::{process::exit, str::FromStr};

use crate::command::Command;

pub(crate) struct Exit {
    exit_code: i32,
}

impl FromStr for Exit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let exit_code = {
            let mut iter = s.split(' ').map(|el| el.trim());
            let (Some("exit"), Some(second)) = (iter.next(), iter.next()) else {
                return Err(())
            };
            second.parse().map_err(|_| ())
        }?;
        Ok(Self { exit_code })
    }
}

impl Command for Exit {
    fn execute(&self) -> Result<(), String> {
        exit(self.exit_code)
    }
}
