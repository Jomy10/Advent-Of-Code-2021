/// The full command for the submarine
pub struct SubCommand {
    pub command: Command,
    pub amt: isize
}
impl SubCommand {
    pub fn new(command: Command, amt: isize) -> SubCommand {
        SubCommand {command, amt}
    }
}

pub enum Command {
    Up,
    Down,
    Forward
}

pub struct CommandParser;
impl CommandParser {
    /// Parses a command in the form of a string to a format a `Submarine` can follow.
    pub fn parse(command: &str) -> SubCommand {
        let mut command = command.split(" ");
        let comm1 = command.next().unwrap().trim();
        match comm1 {
            "forward" => SubCommand::new(Command::Forward, (command.next().unwrap().trim()).parse::<isize>().unwrap()),
            "down" => SubCommand::new(Command::Down, (command.next().unwrap().trim()).parse::<isize>().unwrap()),
            "up" => SubCommand::new(Command::Up, (command.next().unwrap().trim()).parse::<isize>().unwrap()),
            &_ => panic!("The {} command does not exist", comm1)
        }
    }
}