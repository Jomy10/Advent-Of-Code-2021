//! day 8
//!
//! I'll be honest here, the assignment for part 2 is very unclear. I have no idea where to start or
//! what is asked of me to do.
//!
//!
//! comment from *nenoin* on Reddit:
//!
//! The 10 inputs are the “signals” that represent the numbers 0-9. They aren’t in order. Part one
//! shows you that some of these signals can be immediately mapped to a number because of their length.
//! The others are ambiguous, and the problem is figuring out how to narrow down those possibilities.
//!
//! The four signals you have at the end (“outputs”) are a sequence of the same signals you got in
//! the input. You don’t even need to look at these until you’ve figured out which input signal maps
//! to which 0-9 number.
//!
//! > *AdequateElderberry*
//! >
//! > So I have to "crack" the left side 10-tuples (which always represent 0-9 in some order) first
//! > without even caring for the right side and only when this is done I look at the right side,
//! > translating them with what I've found on the left side...
//!
//! -> Thank you for the clear explanation!
//!
//! Now that the assignment was clear, I got the right answer straight away! (NOTE: this solution
//! definitely isn't the prettiest one)

use std::collections::HashMap;
use std::fs;
use colored::*;

// Part 2
fn main() {
    let input = fs::read_to_string("src/inputs/day8.txt").unwrap();
    let input: Vec<(&str, &str)> = input.lines().map(|line| {
        let vec = line.split("|").collect::<Vec<&str>>();
        let vec = vec.into_iter().map(|val| val.trim()).collect::<Vec<&str>>();
        (vec[0], vec[1])
    }).collect();
    
    let mut sum = 0;
    input.iter().for_each(|input_line| {
        println!("ANOTHER ONE");
        // Decipher digits
        // (digit, wires)
        let mut digits: HashMap<u8, String> = HashMap::new();
        
        let set: Vec<&str> = input_line.0.split(" ").collect();
        
        // find 1, 4, 7, 8
        set.iter().for_each(|current_digit| {
            match (current_digit, current_digit.len()) {
                // 1
                (s, 2) => { digits.insert(1, s.to_string()); }
                // 4
                (s, 4) => { digits.insert(4, s.to_string()); }
                // 7
                (s, 3) => { digits.insert(7, s.to_string()); }
                // 8
                (s, 7) => { digits.insert(8, s.to_string()); }
                _ => {}
            };
        });
        
        // find 2, 3, 5 (have 5 digits)
        set.iter().for_each(|current_digit| {
            if current_digit.len() == 5 {
                let signals_of_one = digits.get(&1).unwrap().chars().collect::<Vec<char>>();
                
                if  current_digit.contains(signals_of_one[0]) &&
                    current_digit.contains(signals_of_one[1])
                {   // Then it is 3
                    digits.insert(3, current_digit.to_string());
                }
                // If contains 3/4 signals from 4, then it is 5
                else {
                    let mut matches_for_5 = 0;
                    for signal_of_4 in digits.get(&4).unwrap().chars() {
                        if current_digit.contains(signal_of_4) {
                            matches_for_5 += 1;
                        }
                    }
                    
                    if matches_for_5 >= 3 {
                        digits.insert(5, current_digit.to_string());
                    } else {
                        // It is a 2
                        digits.insert(2, current_digit.to_string());
                    }
                }
            } // Find 0, 6, 9
            else if current_digit.len() == 6 {
                // If contains all of 4, then it is 9
                let mut matches_of_4 = 0;
                for signal_of_4 in digits.get(&4).unwrap().chars() {
                    if current_digit.contains(signal_of_4) {
                        matches_of_4 += 1;
                    }
                }
                
                if matches_of_4 == 4 {
                    digits.insert(9, current_digit.to_string());
                } else {
                    // 0 and 6
                    // if contains all of 1, then it is 0
                    let signals_of_one = digits.get(&1).unwrap().chars().collect::<Vec<char>>();
                    if  current_digit.contains(signals_of_one[0]) &&
                        current_digit.contains(signals_of_one[1])
                    {
                        digits.insert(0, current_digit.to_string());
                    } else {
                        // it is a 6
                        digits.insert(6, current_digit.to_string());
                    }
                }
            }
        });
        
        // All digits have been found
        // Now, decode the right side
        let to_decode: Vec<&str> = input_line.1.split(" ").collect();
        
        println!("{}", format!("{:?}", digits).cyan());
        println!("{}", format!("{:?}", to_decode).green());
        // first, sort digits (wires) and to_decode's values.
        let digits_vec = digits.into_iter().map(|digit| {
            let wires = &digit.1;
            let mut wires = wires.chars().collect::<Vec<char>>();
            wires.sort();
            let mut sorted_wires = String::new();
            wires.iter().for_each(|char| { sorted_wires = format!("{}{}", sorted_wires, char); });
            
            (digit.0 , sorted_wires)
        }).collect::<Vec<(u8, String)>>();
        
        let mut digits: HashMap<u8, String> = HashMap::new();
        digits_vec.into_iter().for_each(|digit| {
            digits.insert(digit.0, digit.1);
        });
        
        
        let to_decode = to_decode.into_iter().map(|wires| {
            let mut wires = wires.chars().collect::<Vec<char>>();
            wires.sort();
            let mut sorted_wires = String::new();
            wires.iter().for_each(|char| { sorted_wires = format!("{}{}", sorted_wires, char); });
            
            sorted_wires
        }).collect::<Vec<String>>();
        
        let decoded: Vec<u8> = to_decode.iter().map(|to_dec| {
            for digit in &digits {
                if to_dec == digit.1 {
                    return digit.0.to_owned();
                }
            }
            
            panic!("That wire pattern does not exist! {:?}, {}", digits, format!("{}",to_dec).green());
        }).collect();
        
        let mut decoded_str = String::new();
        
        for d in decoded {
            decoded_str = format!("{}{}", decoded_str, d);
        }
        
        let decoded = decoded_str.parse::<u32>().unwrap();
        sum += decoded;
    });
    
    println!("{}", sum);
}

#[allow(dead_code)]
fn part1() {
    let input = fs::read_to_string("src/inputs/day8.txt").unwrap();
    let input: Vec<(&str, &str)> = input.lines().map(|line| {
        let vec = line.split("|").collect::<Vec<&str>>();
        let vec = vec.into_iter().map(|val| val.trim()).collect::<Vec<&str>>();
        (vec[0], vec[1])
    }).collect();
    
    // 1, 4, 7, 8
    let mut value_amts = [0; 4];
    
    input.into_iter().for_each(|val| {
        let digits: Vec<&str> = val.1.split(" ").collect();
        for digit in digits {
            match digit.len() {
                2 => value_amts[0] += 1,
                4 => value_amts[1] += 1,
                3 => value_amts[2] += 1,
                7 => value_amts[3] += 1,
                _ => {}
            }
        }
    });
}