use std::fmt::{Debug, Formatter};
use std::fs;
use std::mem::MaybeUninit;

fn main() {
    println!("Total flashes: {}", get_total_flashes());
    println!("Sync step: {}", get_synced_flash_step());
}

struct Octopus {
    /// 0-9
    energy_level: u8,
    /// True if the octopus flashed in this step
    flashed: bool
}

impl Octopus {
    pub fn new(energy_level: u8) -> Self {
        Self {
            energy_level,
            flashed: false
        }
    }
}

impl Debug for Octopus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{} - {}", self.energy_level, self.flashed)
        write!(f, "{}", self.energy_level)
    }
}

trait ArrayDebug {
    fn pdbg(&self) -> String;
}

impl ArrayDebug for [[Octopus; 10]; 10] {
    fn pdbg(&self) -> String {
        let mut output = String::new();
        self.iter().for_each(|row| {
            row.iter().for_each(|cell| {
                output.push_str(&format!("{}{:?}{}",
                    if cell.flashed { "!" } else { " " },
                    cell.energy_level,
                    if cell.flashed {
                        if cell.energy_level.to_string().chars().count() == 2 {
                            "!"
                        } else {
                            " !"
                        }
                    } else {
                        if cell.energy_level.to_string().chars().count() == 2 {
                            " "
                        } else {
                            "  "
                        }
                    }
                ))
            });
            output.push_str("\n");
        });
        output
    }
}

fn read_input() -> [[Octopus; 10]; 10] {
    let mut grid: [[Octopus; 10]; 10] = unsafe { MaybeUninit::uninit().assume_init() };
    let input = fs::read_to_string("input.txt").unwrap();
    let mut row = 0;
    input.chars().filter(|char| char.is_digit(10) ).enumerate().for_each(|(idx, char)| {
        let mut idx = idx;
        while idx > 9 {
            idx -= 10;
        }
        grid[row][idx] = Octopus::new(unsafe { char.to_digit(10).unwrap_unchecked() } as u8);
        if idx == 9 {
            row += 1;
        }
    });
    grid
}

/// Let's octopuses flash, return whether at least one octopus has flashed
fn let_flash(grid: &mut [[Octopus; 10]; 10]) -> u32 {
    let mut flashes_this_round = 0;
    (0..10).for_each(|idx_row| {
        (0..10).for_each(|idx_col| {
            if grid[idx_row][idx_col].energy_level > 9 && !grid[idx_row][idx_col].flashed {
                // Flash
                flashes_this_round += 1;
                grid[idx_row][idx_col].flashed = true;
                (0..=2 as i32).map(|val| val - 1).for_each(|row_add| {
                    (0..=2 as i32).map(|val| val -1).for_each(|col_add| {
                        let row = idx_row as i32 + row_add;
                        let col = idx_col as i32 + col_add;
                        if  // Prevent index out of bouns
                            !(row < 0) && !(col < 0) && !(row > 9) && !(col > 9) &&
                            // Don't add to own energy level
                            !(row_add == 0 && col_add == 0)
                        {
                            grid[(row) as usize][(col) as usize].energy_level += 1;
                        }
                    });
                });
            }
        });
    });
    flashes_this_round
}

/// Answer to part 1
fn get_total_flashes() -> u32 {
    let mut grid = read_input();
    let mut total_flashes = 0;
    (0..100).for_each(|_step| {
        // 1. Increase energy level
        {
            grid.iter_mut().for_each(|row| {
                row.iter_mut().for_each(|cell| {
                    cell.energy_level += 1;
                });
            });
        }
        
        // 2
        loop {
            let flashes = let_flash(&mut grid);
            if flashes == 0 {
                break;
            } else {
                total_flashes += flashes;
            }
        }
        
        // 3
        grid.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|cell| {
                if cell.flashed {
                    cell.flashed = false;
                    cell.energy_level = 0;
                }
            });
        });
    });
    total_flashes
}

/// Get the step at which all octopuses flash simultaneously. Answer to part 2
fn get_synced_flash_step() -> usize {
    let mut grid = read_input();
    let mut total_flashes = 0;
    (1..usize::MAX).find_map(|step| {
        // 1. Increase energy level
        {
            grid.iter_mut().for_each(|row| {
                row.iter_mut().for_each(|cell| {
                    cell.energy_level += 1;
                });
            });
        }
        
        // 2
        loop {
            let flashes = let_flash(&mut grid);
            if flashes == 0 {
                break;
            } else {
                total_flashes += flashes;
            }
        }
        
        // 3
        let mut amount_that_flashed = 0;
        grid.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|cell| {
                if cell.flashed {
                    amount_that_flashed += 1;
                    cell.flashed = false;
                    cell.energy_level = 0;
                }
            });
        });
        return if amount_that_flashed == 100 {
            Some(step)
        } else {
            None
        }
    }).expect("Could not find sync step")
}

// Steps
// 1. energy level += 1
// 2. if energy level > 9:
//      flash
//      energy level adjacent += 1 (incl diag)
//      Check for energy level > 9 again
// 3. If flashed, set energy level to 0
//
// How many flashes after 100 steps?