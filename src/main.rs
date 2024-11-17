use std::io;
use std::io::Write;
use std::process;
use std::env;

fn exit(exit_code: i32) -> !{
    process::exit(exit_code);
}

fn echo(args: Vec<String>) -> i32 {
    println!("{}", &args[1..].join("-"));
    0
}

fn cd(args: Vec<String>) -> i32 {
    if args.len() > 2 {
        eprintln!("too many arguments");
        return 1;
    } else if args.len() == 2 {
        match env::set_current_dir(&args[1]) {
            Err(_) => {
                eprintln!("cannot cd into {}", args[1]);
                return 1;
            },
            Ok(_) => (),
        }
        return 0;
    } else {
        match env::set_current_dir("~") {
            Err(_) => {
                eprintln!("cannot cd into {}", args[1]);
                return 1;
            },
            Ok(_) => (),
        }
        return 0;
    }
}

fn exec_command(args: Vec<String>) -> Result<i32, ()> {
    let mut command: process::Command = process::Command::new(&args[0]);
    if args.len() < 2 {
        match command.status() {
            Err(_) => eprintln!("could not execute {}", args[0]),
            Ok(exit_status) => return Ok(exit_status.code().expect("failed to wait for command")),
        }
    } else {
        match command.args(&args[1..]).status() {
            Err(_) => eprintln!("could not execute {}", args[0]),
            Ok(exit_status) => return Ok(exit_status.code().expect("failed to wait for command")),
        }
    }
    Err(())
}

fn parse_args(line: &str) -> Vec<String> {
    let mut argv: Vec<String> = vec!();
    if line.matches("\"").count() % 2 == 1 || line.matches("'").count() % 2 == 1 {
        // Todo: improve this by printing the index of the starting unterminated quote
        eprintln!("unterminated quoted string");
        return argv;
    // No need to split args if there aren't spaces
    } else if line.matches(" ").count() == 0 {
        return argv;
    } else {
        let mut argument: String = String::new();
        let mut inside_quoted_string: bool = false;
        let mut quote_type: char = ' ';
        for c in line.chars() {
            if c != ' ' || inside_quoted_string {
                // Handle quoted strings (don't ignore spaces)

                // Start the quoted string
                if (c == '\'' || c == '"') && !inside_quoted_string {
                    inside_quoted_string = !inside_quoted_string;
                    quote_type = c;
                    continue; // Ignore the quote
                }

                // End the quoted string
                if (c == '\'' || c == '"') && inside_quoted_string && c == quote_type {
                    inside_quoted_string = !inside_quoted_string;
                    quote_type = ' ';
                    continue;
                }

                print!("{}", c);
                argument.push(c);
            } else {
                argv.push(argument);
                argument = String::new();
            }
        }
        argv.push(argument);
        argv
    }
}

fn main() {
    let stdin  = io::stdin();

    let mut line: String = String::new();
    loop {
        print!("rash$ ");
        io::stdout().flush().expect("closed stdout");

        stdin.read_line(&mut line).expect("lost stdin");

        let line_args: Vec<String> = parse_args(line.trim());
        println!("{:#?}", line_args); // DEBUG

        if line_args.is_empty() {
            continue;
        }

        let command_exit_code: i32 = match line_args[0].as_str() {
            "echo" => echo(line_args),
            "cd" => cd(line_args),
            "exit" => {
                let provided_exit_code: i32 = if line_args.len() > 1 {
                    match line_args[1].parse() {
                        Err(_) => 0,
                        Ok(val) => val,
                    }
                } else {
                    0
                };
                exit(provided_exit_code);
            },
            "bye" => {
                exit(0);
            },
            _ => match exec_command(line_args) {
                Ok(val) => val,
                Err(_) => continue,
            },
        };

        // DEBUG
        eprintln!("command finished with exit code {}", command_exit_code);

        line.clear();
    }
}
