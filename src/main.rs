#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        input = input.trim().to_string();

        let parts: Vec<&str> = input.split_ascii_whitespace().collect();
        let command: &str = parts[0];
        let args: &[&str] = &parts.as_slice()[1..];

        match command {
            "exit" => {
                if args.len() == 1 {
                    let code: i32 = args[0]
                        .parse()
                        .expect("Code for exit must be an integer, e.g. exit 1");
                    exit(code);
                }
            }
            "echo" => println!("{}", args.join(" ")),
            _ => println!("{command}: command not found"),
        }
    }
}
