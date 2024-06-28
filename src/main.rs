use clap::{App, Arg};
use rustyline::error::ReadlineError;
use rustyline::{Editor, Config};
use rustyline::config::OutputStreamType;
use std::process;

fn main() {
    let matches = App::new("Readline Prompt")
        .about("Displays an interactive prompt on stderr and reads user input")
        .arg(Arg::new("prompt")
            .help("Sets the prompt text (default: '> ')")
            .required(false)
            .index(1))
        .arg(Arg::new("default_value")
            .help("Sets a default value for the input")
            .required(false)
            .index(2))
        .after_help("The prompt is written to stderr, and the user's input is output to stdout.")
        .usage("readline_prompt [prompt default_value]")
        .get_matches();

    let prompt = matches.value_of("prompt").unwrap_or("> ");
    let default_value = matches.value_of("default_value").unwrap_or("");

    // Create a custom configuration to use stderr for the prompt
    let config = Config::builder()
        .output_stream(OutputStreamType::Stderr)
        .build();

    // Create the Editor with the custom configuration
    let mut rl = Editor::<()>::with_config(config);

    let readline = rl.readline_with_initial(&format!("{}", prompt), (default_value, ""));

    match readline {
        Ok(line) => {
            // Output the user's input to stdout
            println!("{}", line);
            process::exit(0);
        },
        Err(ReadlineError::Interrupted) => {
            eprintln!("Interrupted");
            process::exit(1);
        },
        Err(ReadlineError::Eof) => {
            eprintln!("EOF");
            process::exit(1);
        },
        Err(err) => {
            eprintln!("Error: {:?}", err);
            process::exit(1);
        }
    }
}
