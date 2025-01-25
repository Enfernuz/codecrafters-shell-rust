#[allow(unused_imports)]
use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::{collections::HashMap, collections::HashSet, process::exit};

fn main() {
    let builtins: HashSet<&str> = HashSet::from(["exit", "echo", "type"]);
    let executables: HashMap<String, String> = get_path_executables();

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
            "type" => handle_type(args, &builtins, &executables),
            _ => println!("{command}: command not found"),
        }
    }
}

fn handle_type(args: &[&str], builtins: &HashSet<&str>, executables: &HashMap<String, String>) {
    if args.len() == 1 {
        let cmd: &str = args[0];
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
        Err(why) => panic!("Error while reading {path}: {}", why.kind()),
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
