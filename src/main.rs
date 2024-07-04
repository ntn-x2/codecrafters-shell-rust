use std::io::{stdin, stdout, Write};

use shell_starter_rust::parse_command;

fn main() {
    let mut stdin = stdin().lock();
    let mut stdout = stdout();
    loop {
        print_prompt(&mut stdout);
        match parse_command(&mut stdin) {
            Ok(command) => {
                command.execute().unwrap();
            }
            Err(error_message) => {
                stdout.write_all(error_message.as_bytes()).unwrap();
                stdout.write_all(b"\n").unwrap();
            }
        };
    }
}

fn print_prompt<Output: Write>(output: &mut Output) {
    output.write_all(b"$ ").unwrap();
    output.flush().unwrap();
}
