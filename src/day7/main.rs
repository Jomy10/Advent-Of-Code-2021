//! day 7
//!       __________...----..____..-'``-..___
//!     ,'.                                  ```--.._
//!    :                                             ``._
//!    |                           --                    ``.
//!    |                   -.-      -.     -   -.        `.
//!    :                     __           --            .     \
//!     `._____________     (  `.   -.-      --  -   .   `     \
//!        `-----------------\   \_.--------..__..--.._ `. `.   :
//!                           `--'     SSt             `-._ .   |
//!                                                        `.`  |
//!                                                          \` |
//!                                                           \ |
//!                                                           / \`.
//!                                                          /  _\-'
//!                                                         /_,'

use std::fs;

fn main() {
	let init_crab_pos: Vec<u32> = fs::read_to_string("src/inputs/day7.txt").unwrap()
        .split(",")
        .map(|str| str.parse::<u32>().unwrap())
        .collect();
    
    let min = init_crab_pos.iter().min().unwrap().clone();
    let max = init_crab_pos.iter().max().unwrap().clone();
    
    // (pos, fuel_cons)
    let mut fuel_consumption: Vec<u32> = Vec::new();
    for pos in min..max {
        let total_fuel_cons = init_crab_pos.iter().map(|crab_pos| {
            // PART 2: This is inefficient, but I can't be bothered to optimize it
            let mut diff: i32 = (*crab_pos as i32) - (pos as i32);
            if diff < 0 {
                diff = -1 * diff;
            }
            let fuel_cons = (1..diff+1).sum::<i32>();
            
            // PART 1
            // let mut fuel_cons: i32 = (*crab_pos as i32) - (pos as i32);
            // if fuel_cons < 0 {
            //     fuel_cons = -1 * fuel_cons;
            // }
            // fuel_cons as u32
            fuel_cons as u32
        }).sum::<u32>();
    
        fuel_consumption.push(total_fuel_cons);
    }
    
    println!("{}", fuel_consumption.iter().min().unwrap());
}