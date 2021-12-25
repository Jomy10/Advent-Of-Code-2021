//! Day 5: Hydrothermal ventures

use std::cmp::max;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::io::Write;
use grid::grid;
use grid::Grid as Gr;

fn main() {
	let input = fs::read_to_string("src/inputs/day5.txt").unwrap();
    let input = input.lines();
    let lines: Vec<Line> = input.into_iter().map(|line| {
        Line::from_str(line)
    }).collect();
    
    // let lines2 = [
    //     Line::new(Point::new(3,3), Point::new(3,7)),
    //     Line::new(Point::new(1,1), Point::new(2,2)),
    //     Line::new(Point::new(0,0), Point::new(100,100)),
    // ].to_vec();
    
    let mut grid = Grid::new(lines);
    grid.populate_grid();
    let overlap = grid.amt_overlap();
    println!("Overlap: {}", overlap);
    // println!("{:?}", grid.grid);
    
    // Write grid to file
    let _ = fs::write("src/outputs/day5.txt", "");
    
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("src/outputs/day5.txt")
        .unwrap();
    
    for r in 0..grid.grid.size().0 {
        file.write_all(
            &grid.grid[r].iter().map(|c| {
                c.to_owned() as u8
            })
            .collect::<Vec<u8>>()
        );
        file.write_all(b"\n");
        // write!(file, "{:?}\n", grid.grid[r]);
    }
}

#[derive(Debug)]
struct Grid {
    grid: Gr<char>,
    lines: Vec<Line>
}

impl Grid {
    pub fn new(lines: Vec<Line>) -> Grid {
        let mut grid = Gr::<char>::new(1000, 1000);
        for r in 0..grid.size().0 {
            for c in 0..grid.size().1 {
                grid[r][c] = '.';
            }
        }
        
        Grid {
            grid,
            lines
        }
    }
    
    /// Draws all the lines to the grid
    pub fn populate_grid(&mut self) {
        for line in self.lines.clone() {
            self.draw_line(line);
        }
    }
    
    /// Draws the lines to the grid
    fn draw_line(&mut self, line: Line) {
        // x axis is constant
        if line.p1.x == line.p2.x {
            let x: usize = line.p1.x as usize;
            
            if line.p1.y < line.p2.y {
                // draw line
                for y in line.p1.y..line.p2.y+1 {
                    let y = y as usize;
                    if self.grid[x][y] == '.' {
                       self.grid[x][y] = '1';
                    } else {
                        self.grid[x][y] = char::from_digit(((self.grid[x][y] as u32 - 0x30) + 1), 10).unwrap();
                    }
                }
            } else {
                // draw line
                for y in line.p2.y..line.p1.y+1 {
                    let y = y as usize;
                    if self.grid[x][y] == '.' {
                        self.grid[x][y] = '1';
                    } else {
                        // convert char to int, add 1 and convert to char in decimal (10)
                        self.grid[x][y] = char::from_digit(((self.grid[x][y] as u32 - 0x30) + 1), 10).unwrap();
                    }
                }
            }
        }
        else if line.p1.y == line.p2.y {
            let y: usize = line.p1.y as usize;
    
            if line.p1.x < line.p2.x {
                // draw line
                for x in line.p1.x..line.p2.x+1 {
                    let x = x as usize;
                    if self.grid[x][y] == '.' {
                        self.grid[x][y] = '1';
                    } else {
                        self.grid[x][y] = char::from_digit(((self.grid[x][y] as u32 - 0x30) + 1), 10).unwrap();
                    }
                }
            } else {
                // draw line
                for x in line.p2.x..line.p1.x+1 {
                    let x: usize = x as usize;
                    if self.grid[x][y] == '.' {
                        self.grid[x][y] = '1';
                    } else {
                        self.grid[x][y] = char::from_digit(((self.grid[x][y] as u32 - 0x30) + 1), 10).unwrap();
                    }
                }
            }
        }
        // Part 2
        else {
            // Diagonal lines
            let diff = (line.p1.x as i32-line.p2.x as i32);
            let mut _ref = 0;
            if diff > 0 {
                _ref = diff;
            } else if diff < 0 {
                _ref = -diff;
            } else {
                panic!("A vertical line is being processed in the diagonal lines.");
            }
            
            let operator_x = if diff > 0 { Operator::Min } else { Operator::Plus };
            let operator_y = if line.p1.y > line.p2.y { Operator::Min } else { Operator::Plus };
            
            let mut x = line.p1.x as usize;
            let mut y = line.p1.y as usize;
            println!("ref: {} | ({}, {})", _ref, line.p1.x, line.p2.x);
            for i in 0.._ref+1 {
                if self.grid[x][y] == '.' {
                    self.grid[x][y] = '1';
                } else {
                    self.grid[x][y] = char::from_digit(((self.grid[x][y] as u32 - 0x30) + 1), 10).unwrap();
                }
                
                x = match operator_x {
                    Operator::Plus => x + 1,
                    Operator::Min => x - 1,
                };
    
                y = match operator_y {
                    Operator::Plus => y + 1,
                    Operator::Min => y - 1,
                };
            }
        }
    }
    
    /// Returns the amount of overlapping points in the grid
    pub fn amt_overlap(&self) -> u32 {
        let mut amt: u32 = 0;
        self.grid.iter().for_each(|val| {
            // println!("Val: {}", (val.to_owned() as i32 - 0x30));
            // if val != &'.' { // val as i32 -> '.' = -2
            // 0x30 = 0 in ASCII table
            if (val.to_owned() as i32 - 0x30) >= 2 {
                amt += 1;
            }
            // }
        });
        
        amt
    }
}

enum Operator {
    Plus,
    Min
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: u32,
    y: u32
}

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }
}

#[derive(Copy, Clone)]
struct Line {
    p1: Point,
    p2: Point
}

impl Line {
    /// Creates a new line with begin point `p1` and endpoint `p2`.
    pub fn new(p1: Point, p2: Point) -> Line {
        Line { p1, p2 }
    }
    
    /// Creates a new line from a string in format `x1,y1 -> x2,y2`.
    pub fn from_str(s: &str) -> Line {
        let points: Vec<u32> = s.
            split(|c| c == ',' || c == ' ' || c == '-' || c == '>')
            .collect::<Vec<&str>>()
            .into_iter()
            .filter(|item| {
                if item == &"" {
                    false
                } else {
                    true
                }
            })
            .map(|item| {
                item.parse::<u32>().unwrap()
            })
            .collect::<Vec<u32>>();
        
        Line::new(
            Point::new(
                points[0],
                points[1]
            ),
            Point::new(
                points[2],
                points[3]
            )
        )
    }
}

impl Debug for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
"Line: (
    ({},{}),
    ({},{})
)",
            self.p1.x, self.p1.y,
            self.p2.x, self.p2.y
        )
    }
}