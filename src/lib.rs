#[allow(unused_imports)]
use std::io::Write;

pub fn prompt_command() -> String {
    print!("$ ");
    std::io::stdout().flush().unwrap();
    {
        let stdin = std::io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        input
    }
}
