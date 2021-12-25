//! day 8

use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("src/inputs/day8.txt").unwrap();
    let input: Vec<(&str, &str)> = input.lines().map(|line| {
        let vec = line.split("|").collect::<Vec<&str>>();
        let vec = vec.into_iter().map(|val| val.trim()).collect::<Vec<&str>>();
        (vec[0], vec[1])
    }).collect();
    
    let transform: HashMap<char, char> = HashMap::from(
        [
            ('a','d'),
            ('b','e'),
            ('c','a'),
            ('d','f'),
            ('e','g'),
            ('f','b'),
            ('g','c')
        ]
    );
    
    let patterns: HashMap<u8, &str> = HashMap::from(
      [
          (0, "abcefg"),
          (1, "cf"),
          (2, "acdeg"),
          (3, "acdfg"),
          (4, "bcdf"),
          (5, "abcfg"),
          (6, "abdefg"),
          (7, "acf"),
          (8, "abcdefg"),
          (9, "abcdfg")
      ]
    );
    
    println!("{:?}", transform);
    
    // 1, 4, 7, 8
    let mut value_amts = [0; 4];
    
    let mut sum: usize = 0;
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