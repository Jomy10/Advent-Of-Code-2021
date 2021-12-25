//! day 8

use std::fs;

fn main() {
    let input = fs::read_to_string("src/inputs/day8.txt").unwrap();
    let input: Vec<Vec<&str>> = input.lines().map(|line| {
        let vec = line.split("|").collect::<Vec<&str>>();
        vec.into_iter().map(|val| val.trim()).collect::<Vec<&str>>()
    }).collect();
    
    // 1, 4, 7, 8
    let mut value_amts = [0; 4];
    
    input.into_iter().for_each(|val| {
        let digits: Vec<&str> = val[1].split(" ").collect();
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
