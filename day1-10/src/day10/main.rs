#![allow(dead_code)]
//! day10

use std::collections::HashMap;
use std::fs;

fn main() {
	let input = fs::read_to_string("../inputs/day10.txt").unwrap();
    let mut parser = SyntaxParser::new(input.lines());
    
    //////////////////// PART 2 ////////////////////
    // Second attempt:
    // let new_lines = parser.filter_corrupt_lines();
    // let filtered_input: String = new_lines.iter()
    //     .flat_map(|v| v.chars())
    //     .collect();
    // let mut parser = SyntaxParser::new(filtered_input.lines());
    // let missing = parser.get_completion();
    // println!("missing: {:?}", &missing);
    // let mut total_scores: Vec<u64> = missing.into_iter().map(|missing_chars| {
    //     let mut score: u64 = 0;
    //     missing_chars.into_iter().for_each(|val| {
    //         // It's at this moment I realized fucked up
    //         for _ in 0..val.1 {
    //             score *= 5;
    //             score += Character::get_score2(&val.0)
    //         }
    //     });
    //     score
    // }).collect();
    // total_scores.sort();
    //
    // let middle_score = total_scores.len() / 2 + 1;
    //
    // println!("Score: {:?}", total_scores[middle_score]);
    //
    // First attempt:
    // let mut total_scores: Vec<u64> = missing.into_iter().map(|chars| {
    //     let mut score: u64 = 0;
    //     chars.into_iter().for_each(|char| {
    //         score = score * 5;
    //         score += char.get_score2();
    //     });
    //
    //     score
    // }).collect();
    // total_scores.sort();
    //
    // let middle_score = total_scores.len() / 2 + 1;
    
    // WRONG: 4368858673
    // println!("Score: {:?}", total_scores[middle_score]);
    ////////////////////////////////////////////////
    
    //////////////////// PART 1 ////////////////////
    // let chars = parser.get_all_illegal_chars();
    //
    // let sum: u32 = chars.into_iter().map(|char| {
    //     char.get_score1()
    // }).sum();
    //
    // println!("{}", sum);
    ////////////////////////////////////////////////
}

struct SyntaxParser<'a, T: Iterator<Item = &'a str> + Clone> {
    lines: T,
    length: usize
}
impl<'a, T> SyntaxParser<'a, T> where T: Iterator<Item = &'a str> + Clone, Vec<&'a str>: FromIterator<<T as Iterator>::Item> {
    pub fn new(lines: T) -> Self {
        let len = lines.clone();
        SyntaxParser { lines, length: len.count() }
    }
    
    /// Gets all first illegal characters from all lines
    pub fn get_all_illegal_chars(&mut self) -> Vec<Character> {
        let mut vec: Vec<Character> = Vec::new();
        for _ in 0..self.length {
            match self.check_next_line() {
                Ok(()) => {},
                Err(ill_char) => vec.push(ill_char)
            }
        }
        
        vec
    }
    
    /// Checks the next line for the first illegal character
    pub fn check_next_line(&mut self) -> Result<(), Character> {
        let line = self.lines.next().unwrap();
        
        let characters = line.chars();
        
        let mut open: Vec<char> = Vec::new();
        for char in characters {
            if Character::is_open_bracket(&char) {
                open.push(char);
            } else {
                let last_bracket = match open.pop() {
                    Some(val) => val,
                    None => return Err(Character::from_ending_char(char))
                };
                
                if !Character::brackets_match(&last_bracket, &char) {
                    return Err(Character::from_ending_char(char));
                }
            }
        }
        
        Ok(())
    }
    
    /// Return a vector containing all non-illegal lines
    pub fn filter_corrupt_lines(&mut self) -> Vec<String> {
        let mut delete_lines: Vec<bool> = Vec::new();
        let _lines: Vec<&str> = self.lines.clone().collect();
        for _ in 0..self.length {
            match self.check_next_line() {
                Ok(()) => delete_lines.push(false),
                Err(_) => delete_lines.push(true)
            }
        }
        
        let mut new_lines: Vec<String> = Vec::new();
        
        for i in 0..self.length {
            if delete_lines[i] == false {
                new_lines.push(_lines[i].to_owned() + "\n");
            }
        }
        
        // It's at this point I regret making lines of type Lines instead of a vec
        // This is also the time that I decided to refactor the code to use Iterator
        // Which unfortunately did not fix this problem :/
        new_lines
    }
    
    /// Returns all missing characters per line
    pub fn get_completion(&mut self) -> Vec<HashMap<Character, u32> >{
        let mut vec: Vec<HashMap<Character, u32>> = Vec::new();
        for line in self.lines.clone() {
            let mut open: Vec<Character> = Vec::new();
            let mut close: Vec<Character> = Vec::new();
            for char in line.chars() {
                if Character::is_open_bracket(&char) {
                    open.push(Character::from_begin_char(char));
                } else {
                    close.push(Character::from_ending_char(char));
                }
            }
            
            // Count types
            let mut count_open: HashMap<Character, u32> = HashMap::new();
            for o in &open {
                let mut val = count_open.remove(o).unwrap_or_else(|| 0);
                val += 1;
                count_open.insert(o.clone(), val);
            }
            
            close.iter().for_each(|char| {
                let mut val = count_open.remove(char).unwrap_or_else(|| 0);
                val -= 1;
                count_open.insert(char.clone(), val);
            });
            
            vec.push(count_open);
        }
        
        vec
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Character {
    Round,
    Square,
    Curly,
    Arrow
}
impl Character {
    pub fn from_ending(s: &str) -> Self {
        match s {
            ")" => Self::Round,
            "]" => Self::Square,
            "}" => Self::Curly,
            ">" => Self::Arrow,
            e => {
                panic!(
                    "{} is not a valid character. Valid characters are {:?}.",
                    e,
                    Self::VALUES
                        .iter()
                        .map(|val| val.to_ending_string())
                        .collect::<Vec<&str>>()
                )
            }
        }
    }
    
    pub fn from_ending_char(c: char) -> Self {
        Self::from_ending(&c.to_string())
    }
    
    pub fn from_begin(s: &str) -> Self {
        match s {
            "(" => Self::Round,
            "[" => Self::Square,
            "{" => Self::Curly,
            "<" => Self::Arrow,
            e => {
                panic!(
                    "{} is not a valid character. Valid characters are {:?}.",
                    e,
                    Self::VALUES
                        .iter()
                        .map(|val| val.to_beginning_string())
                        .collect::<Vec<&str>>()
                )
            }
        }
    }
    
    pub fn from_begin_char(c: char) -> Self {
        Self::from_begin(&c.to_string())
    }
    
    pub const VALUES: [Self; 4] = [Self::Round, Self::Square, Self::Curly, Self::Arrow];
    
    pub fn to_ending_string(&self) -> &str {
        match self {
            Self::Round => ")",
            Self::Square => "]",
            Self::Curly => "}",
            Self::Arrow => ">"
        }
    }
    
    pub fn to_beginning_string(&self) -> &str {
        match self {
            Self::Round => "(",
            Self::Square => "[",
            Self::Curly => "{",
            Self::Arrow => "<"
        }
    }
    
    pub fn get_score1(&self) -> u32 {
        match self {
            Character::Round => 3,
            Character::Square => 57,
            Character::Curly => 1197,
            Character::Arrow => 25137
        }
    }
    
    pub fn get_score2(&self) -> u64 {
        match self {
            Character::Round => 1,
            Character::Square => 2,
            Character::Curly => 3,
            Character::Arrow => 4
        }
    }
}
impl Character {
    pub fn is_open_bracket(char: &char) -> bool {
        if char == &'(' || char == &'[' || char == &'{' || char == &'<' {
            true
        } else {
            false
        }
    }
    
    pub fn brackets_match(open: &char, closing: &char) -> bool {
        if  (open == &'(' && closing == &')') ||
            (open == &'{' && closing == &'}') ||
            (open == &'[' && closing == &']') ||
            (open == &'<' && closing == &'>')
        {
            true
        } else {
            false
        }
    }
}