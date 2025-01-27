#[allow(unused_imports)]
use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, ExitStatus};
use std::{collections::HashMap, collections::HashSet, process::exit};

#[derive(Debug)]
struct UserInput {
    command: String,
    args: Vec<String>,
}

fn main() {
    let builtins: HashSet<&str> = HashSet::from(["exit", "echo", "type", "pwd"]);
    let executables: HashMap<String, String> = get_path_executables();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let user_input = parse_user_input(&input);
        match user_input.command.as_str() {
            "exit" => {
                if user_input.args.len() == 1 {
                    let code: i32 = user_input.args[0]
                        .parse()
                        .expect("Code for exit must be an integer, e.g. exit 1");
                    exit(code);
                }
            }
            "echo" => println!("{}", user_input.args.join(" ")),
            "type" => handle_type(&user_input.args, &builtins, &executables),
            "pwd" => handle_pwd(),
            "cd" => handle_cd(&user_input.args),
            _ => {
                if executables.contains_key(&user_input.command) {
                    exec(&user_input);
                } else {
                    println!("{}: command not found", user_input.command);
                }
            }
        }
    }
}

fn parse_user_input(input: &str) -> UserInput {
    // Use shlex to split the input into parts
    let parts: Vec<String> = shlex::split(input).expect("Invalid input");

    // The first part is the command, and the rest are the arguments
    if parts.is_empty() {
        UserInput {
            command: String::from(""),
            args: Vec::new(),
        }
    } else {
        let command = parts[0].clone();
        let args = parts[1..].to_vec();
        UserInput { command, args }
    }
}

fn exec(user_input: &UserInput) {
    let _status: ExitStatus = Command::new(user_input.command.as_str())
        .args(&user_input.args)
        .status()
        .expect("Program `{program}` failed to execute");
}

fn handle_pwd() {
    println!(
        "{}",
        fs::canonicalize(".")
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
    );
}

fn handle_cd(args: &Vec<String>) {
    if args.len() == 1 {
        let _path: &str = match args[0].as_str() {
            "~" => &env::var("HOME").expect("Could not read the HOME env variable."),
            other => other,
        };
        let path = fs::canonicalize(_path);
        if path.is_err() || env::set_current_dir(path.unwrap().as_path()).is_err() {
            println!("cd: {}: No such file or directory", args[0]);
        }
    } else {
        panic!(
            "The `cd` command should have exactly 1 argument (but got {}), e.g. `cd /usr/bin`",
            args.len()
        );
    }
}

fn handle_type(
    args: &Vec<String>,
    builtins: &HashSet<&str>,
    executables: &HashMap<String, String>,
) {
    if args.len() == 1 {
        let cmd: &str = args[0].as_str();
        if builtins.contains(cmd) {
            println!("{cmd} is a shell builtin");
        } else if executables.contains_key(cmd) {
            println!("{cmd} is {}", executables.get(cmd).unwrap());
        } else {
            println!("{cmd} not found");
        }
    } else {
        panic!("The `type` command must have exactly 1 argument, e.g. `type echo`");
    }
}

fn get_executables_in_dir(path: &str) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
    match fs::read_dir(path) {
        Err(why) => println!("Error while reading {path}: {}", why.kind()),
        Ok(paths) => {
            for path in paths {
                let p = path.unwrap();
                let metadata = p.metadata().unwrap();
                if metadata.is_file() && metadata.permissions().mode() & 0o111 != 0 {
                    result.insert(
                        p.file_name().into_string().unwrap(),
                        p.path().into_os_string().into_string().unwrap(),
                    );
                }
            }
        }
    }
    result
}

fn get_path_executables() -> HashMap<String, String> {
    let path_var: String = env::var("PATH").expect("Could not read the PATH env variable.");

    let mut executables: HashMap<String, String> = HashMap::new();
    for dir in path_var.split(':') {
        for (key, value) in get_executables_in_dir(dir) {
            if !executables.contains_key(&key) {
                executables.insert(key, value);
            }
        }
    }
    executables
}
