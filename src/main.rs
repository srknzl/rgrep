use std::env;
use std::process::exit;
use crate::ParseState::ExpectedOption;

#[derive(Debug)]
struct CommandArgs {
    files: Vec<String>,
    query: String,
    after_context: u32,
    before_context: u32,
    ignore_case: bool,
}

fn option_error_string(option: &str, value: &str) -> String {
    format!(
        "Option {option} got invalid value: {value}",
        option = option,
        value = value
    )
}

struct Option<'a> {
    short_form: &'a str,
    long_form: &'a str,
    default_value: &'a str,
    description: &'a str,
}

struct Category<'a> {
    name: &'a str,
    options: Vec<Option<'a>>,
}

fn print_help() {
    let categories = vec![
        Category {
            name: "Pattern selection and interpretation",
            options: vec![
                Option {
                    short_form: "-i",
                    long_form: "--ignore-case",
                    default_value: "false",
                    description: "ignore case distinctions in patterns and data",
                }
            ],
        },
        Category {
            name: "Context control",
            options: vec![
                Option {
                    short_form: "-A",
                    long_form: "--after-context=NUM",
                    default_value: "0",
                    description: "print NUM lines of trailing context",
                },
                Option {
                    short_form: "-B",
                    long_form: "--before-context=NUM",
                    default_value: "0",
                    description: "print NUM lines of leading context",
                }
            ],
        }
    ];


    let mut help_string = String::from("");

    help_string.push_str("Usage: rgrep [OPTION..] PATTERN FILE [FILE..]\n");
    help_string.push_str("Search for PATTERNS in eacn FILE.\n");
    help_string.push_str("Example: rgrep -i 'hello world' menu.h main.c\n");
    help_string.push_str("\n");

    for category in &categories {
        help_string.push_str(format!("{}:\n", category.name).as_str());
        for option in &category.options {
            help_string.push_str(
                format!("  {short}, {long}  {desc}(default {default})\n",
                        short = option.short_form,
                        long = option.long_form,
                        desc = option.description,
                        default = option.default_value
                ).as_str());
        }
        help_string.push_str("\n");
    }
    println!("{}", help_string);
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
        }
        "i" | "ignore-case" => {
            command_args.ignore_case = true
        }
        _ => {
            return Err(format!("Unexpected option {option}", option = option));
        }
    }
    Ok(())
}

fn requires_value(option: &str) -> Result<bool, String> {
    return match option {
        "-A" | "--after-context" => {
            Ok(true)
        }
        "-B" | "--before-context" => {
            Ok(true)
        }
        "-i" | "--ignore-case" => {
            Ok(false)
        }
        "-h" | "--help" => {
            print_help();
            exit(0);
        }
        _ => {
            Err(format!("Unexpected option {}", option))
        }
    };
}

enum ParseState {
    ExpectedOption = 0,
    ExpectedOptionValue = 1,
    FilenameAndPattern = 2,
}

fn parse_args(args: Vec<String>, command_args: &mut CommandArgs) -> Result<(), String> {
    let mut current_option = String::from("");
    let mut captured_query = false;
    let mut first = true;
    let mut state: &ParseState = &ParseState::ExpectedOption;

    for arg in &args {
        if first { // skip first argument
            first = false;
            continue;
        }

        match state {
            ParseState::ExpectedOption => {
                if arg.starts_with("--") { // long option
                    let split: Vec<&str> = arg.split('=').collect();
                    match split.len() {
                        // does not have =
                        1 => current_option = String::from(&arg[2..]),
                        // has one =
                        2 => {
                            parse_single_option(split[0], split[1], command_args)?
                            state = &ParseState::ExpectedOption
                        },
                        // has more than one =, error
                        _ => return Err(format!("Option {} has more than one equal sign", arg))
                    }
                } else if arg.starts_with('-') {
                    current_option = String::from(&arg[1..]);
                } else { // should be value or filenames
                    match captured_query {
                        // should be filename
                        true => {
                            command_args.files.push(arg.clone());
                        }
                        // should be query
                        false => {
                            command_args.query = arg.clone();
                            captured_query = true;
                        }
                    }
                }
            },
            // expected a value to current option
            ParseState::ExpectedOptionValue => {
                parse_single_option(current_option.as_str(), arg.as_str(), command_args)?;
            },
            _ => {

            }
        }

        if is_option {
            expecting_value = requires_value(arg)?;
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
        ignore_case: false,
    };

    let args: Vec<String> = env::args().collect();
    let args_result = parse_args(args, &mut command_args);
    match args_result {
        Err(x) => {
            eprintln!("{}", x);
            exit(1);
        }
        Ok(_) => { // start operation
            println!("Argument A is {}", command_args.after_context);
            println!("Argument B is {}", command_args.before_context);
            println!("Argument i is {}", command_args.ignore_case);
            println!("Will search {query} in files {:?}",
                     command_args.files,
                     query = command_args.query);
        }
    }
}
