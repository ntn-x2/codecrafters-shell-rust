use std::{
    collections::BTreeSet,
    fs,
    io::{stdout, Write},
    path::PathBuf,
};

use crate::command::{Command, Context, TryWithArgsAndContext};

pub(crate) struct Type {
    command_string: String,
    recognized_commands: BTreeSet<String>,
}

fn path_for(cmd: &str) -> Option<PathBuf> {
    let path_env = env!("PATH");
    let cmd_path = path_env.split(':').find_map(|path| {
        let path = {
            let mut path_buf = PathBuf::new();
            path_buf.push(path);
            path_buf.push(cmd);
            path_buf
        };
        match fs::metadata(&path) {
            Ok(_) => Some(path),
            Err(_) => None,
        }
    })?;
    Some(cmd_path)
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
        } else if let Some(path) = path_for(&self.command_string) {
            stdout
                .write_all(
                    format!(
                        "{} is {}",
                        self.command_string,
                        path.as_os_str().to_str().unwrap()
                    )
                    .as_bytes(),
                )
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
