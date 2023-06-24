extern crate colored;
extern crate crossterm;
extern crate ctrlc;
use colored::control;
use colored::Colorize;
use commands::commands_map;
use crossterm::terminal;
use crossterm::terminal::Clear;
use crossterm::terminal::SetTitle;
use crossterm::{execute, Result};
use std::io::{stdin, stdout, Write};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::channel;

use shared::get_dir;
use shared::set_dir;

mod commands;
mod shared;

fn main() -> Result<()> {
    let _ = control::set_virtual_terminal(true);
    execute!(stdout(), SetTitle("tinshell"))?;
    execute!(stdout(), Clear(terminal::ClearType::All))?;

    let map = commands_map();
    let files_per_row = 7;
    // Only needed for windows
    let (tx, rx) = channel();

    // This effectively prevents closing the application with CTRL-C
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    // Set dir to whatever environment's dir is
    set_dir(&get_dir());
    loop {
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        println!(
            "\n{}",
            String::from(
                "[".to_owned()
                    + ("tinshell".truecolor(48, 153, 117)).to_string().as_str()
                    + "]"
                    + " -> "
                    + get_dir().as_str()
                    + "> "
            )
        );
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(v) => {
                // read_line leaves a trailing newline, which trim removes
                // this needs to be peekable so we can determine when we are on the last command
                let mut commands = input.trim().split(" | ").peekable();
                let mut previous_command = None;

                while let Some(command) = commands.next() {
                    // everything after the first whitespace character is interpreted as args to the command
                    let mut parts = command.trim().split_whitespace();
                    let command = parts.next().unwrap_or("");
                    let args = parts;
                    // Check for command name in hashmap.

                    // if it is a special/reserved command like "exit", match it manually here.
                    match command {
                        "exit" => return Ok(()),
                        command => {}
                    }

                    if (map.contains_key(command)) {
                        // If so, run that command with args.
                        map.get(command).unwrap().command(&args);
                    } else {
                        let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                            Stdio::from(output.stdout.unwrap())
                        });

                        let stdout = if commands.peek().is_some() {
                            // there is another command piped behind this one
                            // prepare to send output to the next command
                            Stdio::piped()
                        } else {
                            // there are no more commands piped behind this one
                            // send output to shell stdout
                            Stdio::inherit()
                        };

                        let output = Command::new(command)
                            .args(args)
                            .stdin(stdin)
                            .stdout(stdout)
                            .spawn();

                        match output {
                            Ok(output) => {
                                previous_command = Some(output);
                            }
                            Err(e) => {
                                previous_command = None;
                                eprintln!("{}", e);
                            }
                        };
                    }
                }

                if let Some(mut final_command) = previous_command {
                    // block until the final command has finished
                    final_command.wait().unwrap();
                }
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    }
}
