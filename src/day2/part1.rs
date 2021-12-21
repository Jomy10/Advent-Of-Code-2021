// use crate::day2::common;
// use crate::day2::common::{Command, SubCommand};
// use crate::json_reader;
//
// fn part1() {
//     let input = json_reader::read_json("src/input_day2.json");
//     let input = &input["commands"];
//
//     // Create a new submarine
//     let mut submarine = Submarine::new();
//     // Execute commands
//     for i in 0..input.len() {
//         let command = input[i].as_str().unwrap();
//         // Parse the command
//         let command = common::CommandParser::parse(command);
//
//         // Move the submarine
//         submarine.execute_commmand(&command);
//     }
//
//     // multiply the pos and depth
//     println!("The answer is: {}", submarine.depth * submarine.pos );
// }
//
// struct Submarine {
//     pos: isize,
//     depth: isize
// }
// impl Submarine {
//     /// Creates a new submarine at the initial position
//     pub fn new() -> Submarine {
//         Submarine{ pos: 0, depth: 0 }
//     }
//
//     pub fn execute_commmand(&mut self, comm: &SubCommand) {
//         match comm.command {
//             Command::Up => { self.move_up(comm.amt) }
//             Command::Down => { self.move_down(comm.amt) }
//             Command::Forward => { self.move_fwd(comm.amt) }
//         }
//     }
//
//     fn move_up(&mut self, amt: isize) {
//         self.depth -= amt;
//     }
//     fn move_down(&mut self, amt: isize) {
//         self.depth += amt;
//     }
//     fn move_fwd(&mut self, amt: isize) {
//         self.pos += amt;
//     }
// }
//
// fn part2() {
//     let input = json_reader::read_json("src/input_day2.json");
//     let input = &input["commands"];
//
//     let mut submarine = Submarine::new();
//     for i in 0..input.len() {
//         let command = CommandParser::parse(input[i].as_str().unwrap());
//         submarine.execute_command(&command);
//     }
//
//     println!("{:?}", submarine.pos * submarine.depth)
// }
//
// #[derive(Default, Debug)]
// struct Submarine {
//     pos: i32,
//     depth: i32,
//     aim: i32
// }
// impl Submarine {
//     pub fn new() -> Submarine {
//         Submarine { ..Default::default() }
//     }
//
//     fn execute_command(&mut self, command: &SubCommand) {
//         match command.command {
//             Command::Up => self.aim -= command.amt as i32,
//             Command::Down => self.aim += command.amt as i32,
//             Command::Forward => {
//                 self.pos += command.amt as i32;
//                 self.depth += self.aim * command.amt as i32;
//             }
//         }
//     }
// }
//
