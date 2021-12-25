//! day 6: Lanternfish
//!
//! Day 6 was pretty easy, however: I should've probably grouped fish with similar age to make this
//! run faster. Because right now it's pretty slow.
//!
//! UPDATE: Optimized day 6 in `new_method()`

use std::fs;

fn main() {
    new_method()
}

fn new_method() {
    const DAYS: usize = 256;
    
    let input = fs::read_to_string("src/inputs/day6.txt").unwrap();
    let input: Vec<&str> = input.split(",").collect();
    let input: Vec<usize> = input.into_iter().map(|val| val.parse::<usize>().unwrap()).collect();
    
    // Represents the amount of fish with a specific lifetime
    let mut fishes: [usize; 9] = [0; 9];
    
    for i in input {
        fishes[i] += 1;
    }
    
    // Simulate
    for _ in 0..DAYS
    {
        // Rotate the array left by one
        fishes.rotate_left(1);
        // The fishes that were at zero -> should be at index 8 (new) and 6
        fishes[6] += fishes[8];
    }
    
    println!("{}", fishes.iter().sum::<usize>());
}

#[allow(dead_code)]
fn old_method() {
    const DAYS: usize = 80;
    
    let input = fs::read_to_string("src/inputs/day6.txt").unwrap();
    let input: Vec<&str> = input.split(",").collect();
    let mut input: Vec<u32> = input.into_iter().map(|val| val.parse::<u32>().unwrap()).collect();
    // let mut input = [1,2,3].to_vec();
    
    
    
    for _ in 0..DAYS {
        let mut new_fish: usize = 0;
        for fish_timer in &mut input {
            if fish_timer == &0 {
                *fish_timer = 6;
                new_fish += 1;
            } else {
                *fish_timer -= 1;
            }
        }
        
        for _ in 0..new_fish {
            input.push(8);
        }
    }
    
    println!("The population is now: {}", input.len());
}