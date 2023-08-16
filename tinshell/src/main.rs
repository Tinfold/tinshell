extern crate colored;
extern crate crossterm;
extern crate nu_ansi_term;
extern crate reedline;
extern crate shared;
extern crate shlex;

use colored::control;
use colored::Colorize;
use commands::commands_map;

use crossterm::terminal::SetTitle;
use crossterm::{execute, Result};
use nu_ansi_term::{Color, Style};
use reedline::{
    DefaultHinter, DefaultPrompt, DefaultPromptSegment, DefaultValidator, Reedline, Signal,
};
use shlex::Shlex;

use shared::get_dir;
use shared::set_dir;
use std::io::{stdout, Write};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::channel;

mod commands;

fn main() -> Result<()> {
    //let completer = Box::new(DefaultCompleter::new(external_commands));
    let validator = Box::new(DefaultValidator);

    let mut line_editor = Reedline::create()
        .with_validator(validator)
        .with_hinter(Box::new(
            DefaultHinter::default().with_style(Style::new().italic().fg(Color::LightGray)),
        ));

    // Eventually need to replace the crappy unicode > character for the prompt. For now, this will do.

    let left_prompt = DefaultPromptSegment::Empty;
    let right_prompt = DefaultPromptSegment::Empty;
    let prompt = DefaultPrompt::new(left_prompt, right_prompt);

    execute!(stdout(), SetTitle("tinshell"))?;
    //execute!(stdout(), Clear(terminal::ClearType::All))?;

    let map = commands_map();

    // Only needed for windows
    let (tx, _rx) = channel();

    // This effectively prevents closing the application with CTRL-C
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    // Set dir to whatever environment's dir is
    set_dir(&get_dir());

    loop {
        // This has to be spammed in the windows terminal to prevent ansi escape codes from breaking
        let _ = control::set_virtual_terminal(true);

        println!(
            "\n{}",
            String::from(
                "[".to_owned()
                    + ("tsh".truecolor(48, 153, 117)).to_string().as_str()
                    + "]"
                    + " -> "
                    + get_dir().as_str()
                    + "> "
            )
        );

        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        //print!("> ");
        stdout().flush().unwrap();

        match line_editor.read_line(&prompt) {
            Ok(Signal::Success(buffer)) => {
                // read_line leaves a trailing newline, which trim removes
                // this needs to be peekable so we can determine when we are on the last command
                let mut commands = buffer.trim().split(" | ").peekable();
                let mut previous_command = None;

                while let Some(command) = commands.next() {
                    // everything after the first whitespace character is interpreted as args to the command
                    let mut parts = Shlex::new(command); //.trim().split_whitespace();
                    let temp = parts.next().unwrap_or(" ".to_string());
                    let command: &str = temp.as_str(); //_or("");
                    let args = parts;

                    // if it is a special/reserved command like "exit", match it manually here.
                    match command {
                        "exit" => return Ok(()),
                        "cd" => {
                            map.get("cd").unwrap().command(args);
                        }
                        _command => {
                            let stdin = previous_command
                                .map_or(Stdio::inherit(), |output: Child| {
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

                    /*
                    let res: bool = map.contains_key(command);
                    println!("{}", command);


                    if res == true {
                        // If so, run that command with args.
                        map.get(command).unwrap().command(args);
                    }
                    */
                }

                if let Some(mut final_command) = previous_command {
                    // block until the final command has finished
                    final_command.wait().unwrap();
                }
            }

            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!("\nAborted!");
            }

            Err(e) => {
                println!("Error: {}", e)
            }
        }
    }
}
