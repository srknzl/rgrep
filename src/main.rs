use std::env;
use std::process::exit;

#[derive(Debug)]
struct CommandArgs {
    files: Vec<String>,
    query: String,
    after_context: u32,
    before_context: u32,
}

fn option_error_string(option: &str, value: &str) -> String {
    format!(
        "Option {option} got invalid value: {value}",
        option = option,
        value = value
    )
}

fn parse_single_option(option: &str, value: &str, command_args: &mut CommandArgs) -> Result<(), String> {
    println!("Parsed option {option} with value {value}", option = option, value = value);
    match option {
        "A" | "after-context" => {
            let result = value.parse::<u32>();
            match result {
                Err(_) => return Err(option_error_string(option, value)),
                Ok(v) => command_args.after_context = v
            }
        }
        "B" | "before-context" => {
            let result = value.parse::<u32>();
            match result {
                Err(_) => return Err(option_error_string(option, value)),
                Ok(v) => command_args.before_context = v
            }
        },
        _ => {
            return Err(format!("Unexpected option {option}", option = option));
        }
    }
    Ok(())
}

fn parse_args(args: Vec<String>, command_args: &mut CommandArgs) -> Result<(), String> {
    let mut current_option = String::from("");
    let mut captured_query = false;
    let mut first = true;

    for arg in args {
        if first { // skip first argument
            first = false;
            continue;
        }

        match current_option.len() {
            0 => {
                if arg.starts_with("--") { // long option
                    let split: Vec<&str> = arg.split('=').collect();
                    match split.len() {
                        // does not have =
                        1 => current_option = String::from(&arg[2..]),
                        // has one =
                        2 => parse_single_option(split[0], split[1], command_args)?,
                        // has more than one =, error
                        _ => return Err(format!("Option {} has more than one equal sign", arg))
                    }
                } else if arg.starts_with('-') {
                    current_option = String::from(&arg[1..])
                } else { // should be value or filenames
                    match captured_query {
                        // should be filename
                        true => {
                            command_args.files.push(arg);
                        }
                        // should be query
                        false => {
                            command_args.query = arg;
                            captured_query = true;
                        }
                    }
                }
            }
            // expected a value
            _ => {
                parse_single_option(current_option.as_str(), arg.as_str(), command_args)?;
                current_option = String::from("");
            }
        }
    }

    Ok(())
}

fn main() {
    let mut command_args: CommandArgs = CommandArgs {
        files: Vec::new(),
        query: String::from(""),
        after_context: 0,
        before_context: 0,
    };

    let args: Vec<String> = env::args().collect();
    let args_result = parse_args(args, &mut command_args);
    match args_result {
        Err(x) => {
            println!("{}", x);
            exit(1);
        }
        Ok(_) => { // start operation
            println!("Argument A is {}", command_args.after_context);
            println!("Argument B is {}", command_args.before_context);
            println!("Will search {query} in files {:?}",
                     command_args.files,
                     query = command_args.query);
        }
    }
}
