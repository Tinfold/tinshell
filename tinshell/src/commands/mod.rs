use std::collections::HashMap;
use Shlex;

pub mod cd;
pub mod del;
pub mod ls;
pub mod md;
pub mod mf;
pub mod mv;
pub mod test;

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
    commands.insert("ls", Command::new(ls::ls));
    commands.insert("cd", Command::new(cd::cd));
    commands.insert("md", Command::new(md::md));
    commands.insert("mv", Command::new(mv::mv));
    commands.insert("del", Command::new(del::del));
    commands.insert("mf", Command::new(mf::mf));
    commands.insert("test", Command::new(test::test));
    return commands;
}
