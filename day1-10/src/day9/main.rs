//! day 9
//!
//! Had some fun with images today

use std::{fs, thread};
use std::fmt::{Display, Formatter};
use grid::Grid;
use colored::*;
// use image::{ImageBuffer, RgbImage};

fn main() {
    let grid: Grid<char> = read_grid("src/inputs/day9.txt");
    // (row, col, val)
    let mut low_points: Vec<(usize, usize, u32)> = Vec::new();
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            let point: char = *grid.get(row, col).unwrap();
            let point = point as u32 - 0x30;
            let mut comparisons: Vec<bool> = Vec::new();
            if col != 0 {
                comparisons.push(*grid.get(row, col-1).unwrap() as u32 - 0x30 > point);
            }
            if col != grid.cols()-1 {
                comparisons.push(*grid.get(row, col+1).unwrap() as u32 - 0x30 > point)
            }
            if row != 0 {
                comparisons.push(*grid.get(row-1, col).unwrap() as u32 - 0x30 > point);
            }
            if row != grid.rows()-1 {
                comparisons.push(*grid.get(row+1, col).unwrap() as u32 - 0x30 > point)
            }
            if !comparisons.contains(&false) {
                low_points.push((row, col, point));
            }
        }
    }
    
    // PART 1: let sum: u32 = low_points.iter().map(|low_point| low_point.2 + 1).sum();
    // println!("{}", sum);
    
    let mut basins: Vec<Vec<(usize, usize, u32)>> = Vec::new();
    
    for low_point in &low_points {
        println!("{}", format!("====={:?}=====", low_point).green());
        basins.push(
            follow_path(low_point, &grid) // size is the amount of points in the vec
        );
    }
    
    let mut basin_sizes: Vec<usize> = basins.iter().map(|basin| calculate_basin_size(basin)).collect();
    println!("basin_sizes: {:?}", basin_sizes);
    
    basin_sizes.sort();
    
    basin_sizes.reverse();
    let result: usize = basin_sizes.iter().take(3).product();
    
    println!("result: {}", result);
    
    /*
    // Representation of the basins
    // let mut image: RgbImage = ImageBuffer::new(grid.cols().to_owned() as u32, grid.rows() as u32);
    // for row in 0..grid.rows() {
    //     for col in 0..grid.cols() {
    //         *image.get_pixel_mut(col as u32, row as u32) = match grid.get(row, col).unwrap() {
    //             &'9' => image::Rgb([0,0,0]),
    //             &'8' => image::Rgb([25,25,25]),
    //             &'7' => image::Rgb([50,50,50]),
    //             &'6' => image::Rgb([100,100,100]),
    //             &'5' => image::Rgb([125,125,125]),
    //             &'4' => image::Rgb([150,150,150]),
    //             &'3' => image::Rgb([200,200,200]),
    //             &'2' => image::Rgb([225,225,225]),
    //             &'1' => image::Rgb([245,245,245]),
    //             &'0' => image::Rgb([255,255,255]),
    //             _ => image::Rgb([255,0,0]),
    //         }
    //     }
    // }
    // image.save("output.png").unwrap();
    // Only represents the borders between the basins
    // let mut image: RgbImage = ImageBuffer::new(grid.cols().to_owned() as u32, grid.rows() as u32);
    // for row in 0..grid.rows() {
    //     for col in 0..grid.cols() {
    //         *image.get_pixel_mut(col as u32, row as u32) = match grid.get(row, col).unwrap() {
    //             &'9' => image::Rgb([0,0,0]),
    //             &'8' => image::Rgb([255,255,255]),
    //             &'7' => image::Rgb([255,255,255]),
    //             &'6' => image::Rgb([255,255,255]),
    //             &'5' => image::Rgb([255,255,255]),
    //             &'4' => image::Rgb([255,255,255]),
    //             &'3' => image::Rgb([255,255,255]),
    //             &'2' => image::Rgb([255,255,255]),
    //             &'1' => image::Rgb([255,255,255]),
    //             &'0' => image::Rgb([255,255,255]),
    //             _ => image::Rgb([255,0,0]),
    //         }
    //     }
    // }
    // image.save("output_borders.png").unwrap();
    */
}

fn calculate_basin_size(basin: &Vec<(usize, usize, u32)>) -> usize {
    let mut filtered: Vec<(usize, usize)> = Vec::new();
    
    for point in basin {
        if !filtered.contains(&(point.0, point.1)) {
            filtered.push((point.0, point.1))
        }
    }
    
    filtered.iter().count() + 1
}

/// Finds the whole basin
fn follow_path(from_point: &(usize, usize, u32), grid: &Grid<char>) -> Vec<(usize, usize, u32)> {
    let row = from_point.0;
    let col = from_point.1;
    let val = from_point.2;
    println!("{}", format!("Following path of point ({}, {}) with value {}", row, col, val).cyan());
    let mut return_vec: Vec<(usize, usize, u32)> = Vec::new();
    let mut adjacent = get_adjacent_basin_points((row, col, val), grid);
    println!("Its adjacent points are {:?}", adjacent);
    if !adjacent.is_empty() {
        for point in &adjacent {
            //thread::spawn(move || {
                return_vec.append(&mut follow_path(&(point.0, point.1, point.2), grid));
            //});
        }
    }
    
    return_vec.append(&mut adjacent);
    
    return return_vec;
}

/// Gets all adjacent points that are part of the same basin of the point defined by `(row, col)`
fn get_adjacent_basin_points(from_point: (usize, usize, u32), grid: &Grid<char>) -> Vec<(usize, usize, u32)> {
    let row = from_point.0;
    let col = from_point.1;
    let val = from_point.2;
    // println!("({}, {})", row, col);
    // (x, y, val, is_part_of_baisin)
    let mut up: (usize, usize, u32, bool) = (0,0,0,false);
    let mut down: (usize, usize, u32, bool) = (0,0,0,false);
    let mut left: (usize, usize, u32, bool) = (0,0,0,false);
    let mut right: (usize, usize, u32, bool) = (0,0,0,false);
    
    if row != 0 {
        let point = grid.get(row - 1, col).unwrap();
        if point != &'9' {
            up = (row - 1, col, point.clone() as u32 - 0x30, true);
        }
    }
    if row != grid.rows()-1 {
        let point = grid.get(row + 1, col).unwrap();
        if point != &'9' {
            down = (row + 1, col, point.clone() as u32 - 0x30, true);
        }
    }
    if col != 0 {
        let point = grid.get(row, col - 1).unwrap();
        if point != &'9' {
            left = (row, col - 1, point.clone() as u32 - 0x30, true);
        }
    }
    if col != grid.cols()-1 {
        let point = grid.get(row, col + 1).unwrap();
        if point != &'9' {
            right = (row, col + 1, point.clone() as u32 - 0x30, true);
        }
    }
    
    let mut points = Vec::<(usize,usize, u32)>::new();
    let mut dirs: Vec<(usize, usize, u32, bool)> = Vec::new();
    println!("Points in dirs: {:?}, {:?}, {:?}, {:?}", up, down, left, right);
    if val < up.2 {
        dirs.push(up);
    }
    if val < down.2 {
        dirs.push(down);
    }
    if val < left.2 {
        dirs.push(left);
    }
    if val < right.2 {
        dirs.push(right);
    }
    
    for point in dirs {
        if point.3 == true {
            points.push((point.0, point.1, point.2))
        }
    }
    
    points
}

fn read_grid(path: &str) -> Grid<char> {
    let input = fs::read_to_string(path).unwrap();
    let input: Vec<&str> = input.lines().collect();
    // let rows = input.len();
    let cols = input[0].len();
    
    let mut rows: Vec<char> = Vec::new();
    for i in 0..input.len() {
        let line = input[i];
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        rows.append(&mut row);
    }
    
    let grid = Grid::from_vec(rows, cols);
    
    grid
}