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
        let mut args = parts;

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
            },
            "yeet" => {
                remove_file(&args);
            },
            "uwumake" => {
                touch_file(&args);
            },
            "spillthetea" => {
                list_directory_contents();
            }, 
            "flexstatus" => {
                git_flex_status();
            },
            "whereami" => {
                reveal_location();
            },        
            "stashthatchat" => {
                stash_all_changes();
            },         
            "spillthattea" => {
                let message = collect_commit_message(&mut args);
                commit_changes(&message);
            },
            "exploreuniverse" => {
                explore_universe();
            },                                                      
            command => {
                let child = Command::new(command)
                    .args(args)
                    .spawn();

                match child {
                    Ok(mut child) => {
                        child.wait().expect("Command failed to run");
                    }
                    Err(e) => {
                        eprintln!("Failed to execute command: {}", e);
                    }
                }
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

fn remove_file(args: &std::str::SplitWhitespace<'_>) {
    println!("Yeeting file! 🎯");
    if let Some(file) = args.clone().peekable().peek() {
        let result = std::fs::remove_file(file);
        match result {
            Ok(_) => println!("Yeeted {} successfully! 🚀", file),
            Err(e) => eprintln!("Failed to yeet {}: {}", file, e),
        }
    } else {
        println!("No file to yeet!");
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

fn touch_file(args: &std::str::SplitWhitespace<'_>) {
    println!("Touching file! UwU 😘");
    if let Some(file) = args.clone().peekable().peek() {
        let result = std::fs::File::create(file);
        match result {
            Ok(_) => println!("Touched {} successfully! 💞", file),
            Err(e) => eprintln!("Failed to touch {}: {} 😘", file, e),
        }
    } else {
        println!("No file to touch! 😘");
    }
}

fn list_directory_contents() {
    println!("Spilling the tea on all these files and folders! ☕👀");
    match std::fs::read_dir(env::current_dir().expect("Failed to read current directory")) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("{} got the gossip! 📂", entry.path().display().to_string().bright_yellow().bold());
                }
            }
            println!("All tea, all shade! 💅");
        }
        Err(e) => eprintln!("Couldn't spill any tea because: {}", e),
    }
}

fn git_flex_status() {
    println!("Let's flex that Git status! 💪😎");
    match Command::new("git").arg("status").output() {
        Ok(output) => {
            let status = String::from_utf8_lossy(&output.stdout);
            println!(
                "{}",
                format!("Current sitch in the repo: \n{}", status).bright_cyan().bold()
            );
        }
        Err(e) => println!("Git flexing failed: {}", e),
    }
}

fn reveal_location() {
    println!("Unraveling the mysteries of your current vibes... 🧐🌌");
    let current_path = env::current_dir().expect("Failed to discover current location");
    println!("You are here, in this digital universe: {}", current_path.display().to_string().bright_magenta().bold());
    println!("Mind = blown 🤯 Stay woke, navigator! 🚀");
}

fn stash_all_changes() {
    println!("👀 Gathering all the tea... Prepping for the ultimate gossip stash! 💅🔮");
    match Command::new("git").args(["add", "--all"]).output() {
        Ok(_) => {
            println!("✨ All the files are now chilling in the staging area! Ready to spill the tea with a commit? ✨");
        }
        Err(e) => {
            eprintln!("Oops, couldn't stash that chat: {}", e);
        }
    }
}

fn collect_commit_message(args: &mut std::str::SplitWhitespace) -> String {
    args.collect::<Vec<&str>>().join(" ").trim().to_string()
}

fn commit_changes(message: &str) {
    if message.is_empty() {
        println!("Sis, you can't spill the tea without a message! Try again with some real gossip. 🤫💬");
        return;
    }
    
    println!("👑 Preparing the royal scroll of changes... 📜✨");
    match Command::new("git").args(["commit", "-m", message]).output() {
        Ok(output) => {
            if output.status.success() {
                let response = String::from_utf8_lossy(&output.stdout);
                println!("🎉 The tea has been officially spilled: {}\n{}", message, response);
                println!("Drama archived. What's next on the tea menu? 🍵👀");
            } else {
                let error_message = String::from_utf8_lossy(&output.stderr);
                eprintln!("Failed to spill the tea: {}\n{}", message, error_message);
            }
        },
        Err(e) => {
            eprintln!("Couldn't even start to spill the tea: {}", e);
        }
    }
}

fn explore_universe() {
    let messages = [
        "🌠 Did you know? The Milky Way is on a collision course with Andromeda in about 4 billion years. Time to start packing!",
        "🚀 Quote by Carl Sagan: 'Somewhere, something incredible is waiting to be known.'",
        "🌌 Fun Fact: A day on Venus is longer than a year on Venus. Talk about a long workday!",
        "✨ Remember: 'Shoot for the stars, and even if you miss, you'll land among the stars.' - Les Brown",
        "🔭 Did you know? There are more stars in the universe than grains of sand on all the Earth's beaches.",
        "💫 'The cosmos is within us. We are made of star-stuff.' - Carl Sagan",
        "🌍 'Look again at that dot. That's here. That's home. That's us.' - Carl Sagan, referring to the Pale Blue Dot",
        "🌖 'The Moon's gravity affects the ocean tides, but your determination defines your tide of life.' - Zoomer Zen",
    ];
    let random_message = rand::thread_rng().gen_range(0..messages.len());
    println!("{}", messages[random_message].bright_purple().bold());
    println!("🌈 Embrace the mystery of the cosmos and code on, explorer! 🌈");
}