use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::fs::File;
use std::io::Write;
use crossterm::execute;
use uuid::Uuid;

fn main() {
    let rnd = get_rnd("src/inputs/day4-rnd.txt");
    let boards = Board::multiple_from_file("src/inputs/day4-boards.txt");
    
    let mut game = Game::new(rnd, boards);
    
    match game.first_to_win() {
        Ok(winner) => {
            let winning_board = winner.0;
            println!("WINNER = {:?}", winning_board);
            let number = winner.1;
            
            let sum = winning_board[0].sum_unchecked();
            let final_score = sum * number;
    
            println!("The final score is {}", final_score);
        },
        Err(err) => println!("{}", err)
    }
}

#[derive(serde::Serialize)]
struct Game {
    numbers: Vec<u32>,
    boards: Vec<Board>
}

impl Game {
    pub fn new(numbers: Vec<u32>, boards: Vec<Board>) -> Game {
        Game {
            numbers,
            boards,
        }
    }
    
    /// Finds the first board that will win this game
    /// # Returns
    /// - `Ok((Vec<&Board>, u32))`: The board(s) that won and the number at which they won
    pub fn first_to_win(&mut self) -> Result<(Vec<&Board>, u32), &str> {
        // Loop over each round
        let mut excecute_count: u32 = 0;
        for number in &self.numbers.to_owned() {
            // cross out any matching numbers on the boards
            self.cross_numbers(number.to_owned());
            // check for winners
            let winners = self.check_victory();
            // Return winners
            if !winners.is_empty() {
                let mut _winners: Vec<&Board> = Vec::new();
                for winner in winners {
                    _winners.push(self.find_board(winner).unwrap());
                }
                return Ok((_winners, number.to_owned()));
            }
        }
        Err("This game has no winners")
    }
    
    pub fn find_board(&self, id: u128) -> Result<&Board, String> {
        for board in &self.boards {
            if board.id == id {
                return Ok(board);
            }
        }
        Err(format!("The board with id {} does not exist", id))
    }
    
    pub fn cross_numbers(&mut self, number: u32) {
        for board in &mut self.boards {
            board.cross_out(number);
        }
    }
    
    /// Checks if any of the boards have won yet and return their uid
    pub fn check_victory(&self) -> Vec<u128> {
        let mut victorious_boards: Vec<u128> = Vec::new();
        for board in &self.boards {
            let victory = self.check_board_victory(board.id);
            if victory {
                victorious_boards.push(board.id);
            }
        }
        
        victorious_boards
    }
    
    /// Returns true if the board at index i has won, false otherwise
    fn check_board_victory(&self, id: u128) -> bool {
        let board: &Vec<&Board> = &self.boards.iter().filter(|board| board.id == id).collect();
        let board = &board[0];
        // Check horizontally
        for row in board.checked {
            let mut victory = true;
            for number in row {
                if number == 0 {
                    victory = false;
                }
            }
            // One row is full of 1's
            if victory == true {
                return true;
            }
        }
        
        // Check vertically
        for col in 0..board.checked[0].len() {
            let mut victory = true;
            let mut column: Vec<u32> = Vec::new();
            for i in 0..board.checked.len() {
                column.push(board.checked[i][col]);
            }
            
            
            for number in column {
                if number == 0 {
                    victory = false;
                }
            }
            
            if victory {
                return true;
            }
        }
        
        return false;
    }
}


#[derive(Copy, Clone, serde::Serialize)]
struct Board {
    /// A unique id for the board
    pub id: u128,
    pub grid: [[u32; 5]; 5],
    pub checked: [[u32; 5]; 5]
}

impl Board {
    pub fn new() -> Board {
        let init_val = [0,0,0,0,0];
        Board {
            id: Uuid::new_v4().as_u128(),
            grid: [init_val, init_val, init_val, init_val, init_val],
            checked: [init_val, init_val, init_val, init_val, init_val]
        }
    }
    
    /// Crosses out numbers on the `chosen` array that match `number`
    pub fn cross_out(&mut self, number: u32) {
        let mut row_num = 0;
        for row in &self.grid {
            let mut col_num = 0;
            for num in row {
                if num == &number {
                    self.checked[row_num][col_num] = 1;
                }
                col_num += 1;
            }
            row_num += 1;
        }
    }
    
    /// Returns the sum of all the unchecked board numbers
    pub fn sum_unchecked(&self) -> u32 {
        let mut sum: u32 = 0;
        
        let mut _row = 0;
        for row in &self.checked {
            let mut _col = 0;
            for col in row {
                // if col is unchecked
                if col.to_owned() == 0 {
                    println!("{}, {}", col, self.grid[_row][_col]);
                    sum += self.grid[_row][_col];
                }
                _col += 1;
            }
            _row += 1;
        }
        
        sum
    }
    
    pub fn multiple_from_file(file: &str) -> Vec<Board> {
        let file = fs::read_to_string(file).unwrap();
        // Lines in a vector + trim whitespaces
        let lines: Vec<&str> = file.lines().collect::<Vec<&str>>().iter_mut().map(|line| line.trim()).collect();
        
        let mut board_n = 0;
        let mut grid_row = 0;
        let mut boards: Vec<Board> = Vec::new();
        for line in lines {
            // Extract the numbers from the line
            let mut numbers: Vec<&str> = line.split(" ").collect();
            if numbers.len() == 1 {
                // numbers = [""], so ignore
                continue;
            }
            
            if numbers.len() != 5 {
                numbers = numbers.into_iter().filter(|number| if number.to_string() == "".to_string() { false } else { true }).collect();
            }
            
            let mut row: [u32; 5] = [0,0,0,0,0];
            for i in 0..numbers.len() {
                let number: u32 = numbers[i].parse::<u32>().unwrap();
                row[i] = number;
            }
            
            // increase grid_row by one & insert into boards
            if grid_row == 0 {
                // Initialize the board in the vec
                boards.push(Board::new());
            }
            boards[board_n].grid[grid_row] = row;
            grid_row += 1;
            // if grid_row = 5, set grid_row back to 0 and add 1 to board_n
            if grid_row == 5 {
                grid_row = 0;
                board_n += 1;
            }
        }
        
        boards
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Board #{}\n{:?}\t{:?}\n{:?}\t{:?}\n{:?}\t{:?}\n{:?}\t{:?}\n{:?}\t{:?}", self.id,
               self.grid[0], self.checked[0],
               self.grid[1], self.checked[1],
               self.grid[2], self.checked[2],
               self.grid[3], self.checked[3],
               self.grid[4], self.checked[4]
        )
    }
}

/// Retrieve the random numbers from a file
pub fn get_rnd(file: &str) -> Vec<u32> {
    let file = fs::read_to_string(file).unwrap();
    file.split(",").collect::<Vec<&str>>().into_iter().map(|val| val.parse::<u32>().unwrap()).collect()
}