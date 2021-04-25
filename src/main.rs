use std::env;
use std::process::{exit};

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

/// Flags are options that do not take a value
fn parse_flag(option: &str, command_args: &mut CommandArgs) -> Result<(), String> {
    match option {
        "i" | "ignore-case" => {
            command_args.ignore_case = true
        }
        _ => {
            return Err(format!("Unexpected flag {option}", option = option));
        }
    }
    Ok(())
}

/// Options take values
fn parse_non_flag(option: &str, value: &str, command_args: &mut CommandArgs) -> Result<(), String> {
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
        _ => {
            return Err(format!("Unexpected option {option}", option = option));
        }
    }
    Ok(())
}

/// Whether or not option is not flag
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

enum OptionType {
    FLAG = 0,
    NONFLAG = 1,
}

/// Returns error or option type parsed
fn parse_nonflag_or_flag(argument: &str, args_length: usize, index: usize, args: &Vec<String>, command_args: &mut CommandArgs) -> Result<OptionType, String> {
    let requires_value = requires_value(argument)?;
    if requires_value && index + 1 < args_length { // have at least one more argument
        parse_non_flag(argument, args[index + 1].as_str(), command_args)?;
        Ok(OptionType::NONFLAG)
    } else if !requires_value {
        parse_flag(argument, command_args)?;
        Ok(OptionType::FLAG)
    } else {
        return Err(format!("Option {} requires value but no value is passed", argument));
    }
}


fn parse_args(args: Vec<String>, command_args: &mut CommandArgs) -> Result<(), String> {
    // made true after query parsing finished.
    let mut query_parsed = false;

    // start from 1; so, skip the first argument which is the command name
    let mut index = 1;
    while index < args.len() {
        let arg = &args[index];

        if query_parsed {
            command_args.files.push(arg.clone());
        } else {
            if arg.starts_with("--") { // long option
                let split: Vec<&str> = arg.split('=').collect();
                match split.len() {
                    // does not have = sign, we need to take two values
                    1 => {
                        match parse_nonflag_or_flag(arg, args.len(), index, &args, command_args)? {
                            OptionType::NONFLAG => index += 1,
                            OptionType::FLAG => {}
                        }
                    }
                    // has one = sign
                    2 => {
                        parse_non_flag(split[0], split[1], command_args)?;
                    }
                    // has more than one = error
                    _ => return Err(format!("Option {} has more than one equal sign", arg))
                }
            } else if arg.starts_with('-') {
                match parse_nonflag_or_flag(arg, args.len(), index, &args, command_args)? {
                    OptionType::NONFLAG => index += 1,
                    OptionType::FLAG => {}
                }
            } else { // parse query
                command_args.query = arg.clone();
                query_parsed = true;
            }
        }

        index += 1
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
