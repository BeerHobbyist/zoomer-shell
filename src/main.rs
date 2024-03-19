use colored::*;
use rand::Rng;
use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    ctrlc::set_handler(move || {
        println!("\nYou need to pass the vibe check to exit the shell! 😈");
        print_shell_prompt();
    })
    .expect("Error setting Ctrl-C handler");
    println!(
        "{} {} 👌😏😘",
        "ZOOMER SHELL ACTIVATED.".bright_green().bold(),
        "SLAY SIS!".bright_red().bold()
    );
    loop {
        print_shell_prompt();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "nocapgoto" => {
                // default to '/' as new directory if one was not provided
                change_directory(&args);
            }
            "vibecheck" => {
                uptime();
            }
            "ick" => {
                println!("🤢");
                let diff_word = generate_diff_word();
                println!(
                    "Here's a difficult word for you: {}. Type it in to exit the shell!",
                    diff_word
                );
                let mut user_input = String::new();
                stdin()
                    .read_line(&mut user_input)
                    .expect("Failed to read line");
                if user_input.trim() == diff_word {
                    println!("You passed the vibe check! 😎");
                    println!("\n👋 Goodbye! Stay vibin'! 😘");
                    break;
                } else {
                    println!("Wrong word! You're stuck here forever! 😈");
                }
            }
            command => {
                let mut child = Command::new(command)
                    .args(args)
                    .spawn()
                    .expect("Failed to execute command!");

                child.wait().expect("Command failed");
            }
        }
    }
}

fn print_shell_prompt() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    print!(
        "{}{}> ",
        "~".bright_blue().bold(),
        current_dir.display().to_string().bright_blue().bold()
    );
    stdout().flush().expect("Failed to flush stdout");
}

fn change_directory(args: &std::str::SplitWhitespace<'_>) {
    let new_dir = args.clone().peekable().peek().map_or("/", |x| *x);
    let root = Path::new(new_dir);
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e);
    }
}

fn uptime() {
    match Command::new("uptime").output() {
        Ok(output) => {
            let uptime = String::from_utf8_lossy(&output.stdout);
            println!(
                "{} {}",
                "System vibe status 😏:".purple().bold(),
                uptime.trim().bold()
            );
        }
        Err(e) => println!("Failed to check vibe: {}", e),
    }
}

fn generate_diff_word() -> String {
    let diff_words = [
        "Pneumonoultramicroscopicsilicovolcanoconiosis",
        "Hippopotomonstrosesquippedaliophobia",
        "Supercalifragilisticexpialidocious",
        "Floccinaucinihilipilification",
        "Antidisestablishmentarianism",
        "Electroencephalographically",
        "Honorificabilitudinitatibus",
        "Thyroparathyroidectomized",
        "Deinstitutionalization",
        "Incomprehensibilities",
        "Pseudopseudohypoparathyroidism",
    ];
    let random_number = rand::thread_rng().gen_range(0..diff_words.len());
    return diff_words[random_number].to_string();
}
