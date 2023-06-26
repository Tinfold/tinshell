use std::collections::HashMap;
use Shlex;

pub mod cd;

pub struct Command {
    func: fn(Shlex),
}

impl Command {
    fn new(func: fn(Shlex)) -> Command {
        Command { func: func }
    }
    pub fn command(&self, args: Shlex) {
        (self.func)(args);
    }
}

// COMMANDS

pub fn commands_map() -> HashMap<&'static str, Command> {
    let mut commands = HashMap::new();
    commands.insert("cd", Command::new(cd::cd));
    return commands;
}
