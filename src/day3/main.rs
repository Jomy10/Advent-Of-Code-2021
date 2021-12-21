use json::JsonValue;

mod json_reader;

// o2  111110011011
// co2 011010100000
// O2: 3995 CO2: 1696
// result: 6775520

fn main() {
    definite_part2()
}

struct Counts (i32, i32);
fn get_common_bit(data: &Vec<String>, i: usize) -> i32 {
    let mut counts = Counts(0,0);
    
    for d in data {
        if &d[i..i+1] == "1" {
            counts.1 += 1;
        } else if &d[i..i+1] == "0" {
            counts.0 += 1;
        } else {
            panic!("not a valid bit");
        }
    }
    
    if counts.1 >= data.len() as i32 / 2 {
        return 1
    } else if counts.0 > data.len() as i32 / 2 {
        return 0
    } else {
        panic!("How did we get here? {}, {}", counts.1, counts.0)
    }
}

fn bit_criteria(data: &Vec<String>, rating: RatingType, i: usize) -> i32 {
    let common_bit = get_common_bit(data, i);
    
    if rating == RatingType::CO2 {
        return if common_bit == 1 { 0 } else { 1 }
    } else if rating == RatingType::Oxygen {
        return if common_bit == 1 { 1 } else { 0 }
    } else {
        return -1;
    }
}

fn get_rating(data: &Vec<String>, rating: RatingType) -> String {
    let len = data.len();
    
    let mut d = data.clone();
    
    for i in 0..len {
        let bit = bit_criteria(data, rating.clone(), i);
        
        d = d.into_iter().filter(|v| {
            &v[i..i+1] == if bit == 1 { "1" } else { "0" }
        }).collect();
        
        if d.len() == 1 {
            return d[0].to_string();
        }
    }
    
    return "".to_string();
}

fn definite_part2() {
    let data = get_input();
    
    let o2gen = get_rating(&data, RatingType::Oxygen);
    let co2gen = get_rating(&data, RatingType::CO2);
    
    println!("O2: {}, co2: {}", o2gen, co2gen);
    
    let o2gen = bits_to_dec(&o2gen);
    let co2gen = bits_to_dec(&co2gen);
    
    println!("02: {}, CO2: {}\nresult: {}", o2gen, co2gen, o2gen * co2gen);
}

fn _2() {
    let vec_in = get_input();
    
    // Loop over all the bits
    let mut oxygen_vec = vec_in.clone();
    let mut co2 = oxygen_vec.clone();
    for i in 0..12 {
        if oxygen_vec.len() == 1 {
            break;
        }
        let common = most_common(&oxygen_vec, i, RatingType::Oxygen);
        println!("{}", common);
        if common == 1 {
            oxygen_vec = oxygen_vec.into_iter().filter(|v| if &v[i..i+1] != "1" { false } else { true }).collect();
        } else if common == 0 {
            oxygen_vec = oxygen_vec.into_iter().filter(|v| if &v[i..i+1] != "0" { false } else { true }).collect();
        } else {
            panic!("That is not a valid bit");
        }
        println!("level: {:?}", oxygen_vec.len());
    }
    
    println!("== CO2 ==");
    for i in 0..12 {
        if co2.len() == 1 {
            break;
        }
        let common = most_common(&co2, i, RatingType::CO2);
        println!("Most Common: {}", common);
        if common == 1 {
            co2 = co2.into_iter().filter(|v| if &v[i..i+1] != "1" { false } else { true }).collect();
        } else if common == 0 {
            co2 = co2.into_iter().filter(|v| if &v[i..i+1] != "0" { false } else { true }).collect();
        } else {
            panic!("That is not a valid bit");
        }
        // println!("{:?}", co2);
        println!("level: {:?}", co2.len());
    }
    
    println!("{:?} - {:?}", oxygen_vec, co2);
    
    let oxygen = bits_to_dec(&oxygen_vec[0]);
    let co2 = bits_to_dec(&co2[0]);
    println!("Oxygen level: {}\nCo2 rating: {}", oxygen, co2);
    
    println!("Life support rating: {}", oxygen * co2);
}

#[derive(Copy, Clone, Eq, PartialOrd, PartialEq, Ord)]
enum RatingType {
    Oxygen,
    CO2
}
fn most_common(vec: &Vec<String>, index: usize, t: RatingType) -> i32 {
    let mut one: i32 = 0;
    let mut zero: i32 = 0;
    vec.iter().for_each(|v| if &v[index..index+1] == "1" {one += 1} else if &v[index..index+1] == "0" {zero += 1} else { panic!("That is not a valid input bit") } );
    println!("one: {}", one);
    println!("len: {}", vec.len());
    return match t {
        RatingType::Oxygen => if one >= (vec.len() as i32) / 2 { 1 } else { 0 },
        RatingType::CO2 => if one >= (vec.len() as i32) / 2 { 0 } else { 1 }
    }
}

// #[allow(dead_code)]
// fn part2_eh() {
//     // INTERESTING
//     // let stdin = io::stdin();
//     // let diag: Vec<i32> = stdin.lock().lines().flatten().flat_map(|bstr| i32::from_str_radix(&bstr, 2)).collect();
//     const BIT_LEN: i32 = 12;
//     let mut vec_in = get_input();
//     let mut vec_in: Vec<i32> = vec_in.iter().flat_map(|bstr| i32::from_str_radix(&bstr, 2)).collect();
//     println!("{:?}", vec_in);
//
//     let mut i: usize = 1;
//     let mut criteria: i32 = 0;
//
//     loop {
//         // Build o2_bit_mask
//         let vec_size = vec_in.len();
//
//         let num_set: i32 = vec_in.iter().filter(|val| (*val & (1 << BIT_LEN - 1)) > 0).count() as i32;
//         println!("{}", num_set);
//         if num_set >= (vec_size as i32) - num_set {
//             criteria = criteria | 1 << (BIT_LEN - 1);
//         }
//
//
//         break;
//     }
// }
//
// #[allow(dead_code)]
// /// Keep only numbers selected by the **bit criteria**
// ///
// /// # Bit criteria
// fn part2_depr() {
//
//     println!("Getting input...");
//     let arr = get_input();
//
//     println!("Sorting input...");
//     let arr = sort(&arr);
//
//     println!("Determining oxygen level...");
//     let x = determine_oxygen(&arr);
//     println!("{}", x);
//     let oxygen = bits_to_dec(&x);
//     println!("Oxygen level: {}", oxygen);
//
//     println!("Determining CO2 scrubber rating...");
//     let x = determine_co2_scrubber_rating(&arr);
//     println!("{}", x);
//     let co2_rating = bits_to_dec(&x);
//     println!("CO2 scrubber rating: {}", co2_rating);
//
//     // life support rating = oxygen generator rating * CO2 scrubber rating
//     println!("Life support rating: {}", oxygen * co2_rating);
// }
// #[allow(dead_code)]
// fn determine_co2_scrubber_rating(vec: &Vec<String>) -> String {
//     let mut result = vec.clone();
//     for i in 0..vec[0].len() {
//         let least_common = if determine_most_common(&result, i) == 1 { 0 } else { 1 };
//
//         result = search(&result, i, least_common);
//
//         if result.len() == 1 {
//             return result[0].clone();
//         }
//     }
//
//     String::from("Not found")
// }
// #[allow(dead_code)]
// fn determine_oxygen(vec: &Vec<String>) -> String {
//     let mut result = vec.clone();
//     for i in 0..vec[0].len() {
//         // The most common bit at this position
//         let most_common = determine_most_common(&result, i);
//
//         result = search(&result, i, most_common);
//
//         if result.len() == 1 {
//             return result[0].clone();
//         }
//     }
//
//     return String::from("Not found");
// }
// #[allow(dead_code)]
// fn search(vec: &Vec<String>, index: usize, value: i32) -> Vec<String> {
//     let mut mut_vec = vec.clone();
//
//      for i in 0..vec.len() {
//          if vec[i][index..index+1].parse::<i32>().unwrap() != value {
//              let j = mut_vec.binary_search(&vec[i]).unwrap();
//              mut_vec.remove(j);
//          }
//      }
//
//     return mut_vec;
// }
#[allow(dead_code)]
fn sort(vec: &Vec<String>) -> Vec<String> {
    // let tmp = vec.clone();
    let mut vec = vec.clone();
    // println!("{}", vec.len());
    let mut to_increment = 1000;
    loop {
        // print!("Pass {}. {}, {}, {}, {}, ...  ", i, vec[1], vec[2], vec[3], vec[4]);
        let mut sorted = true;
        let mut converted = 0;
        for i in 0..(vec.len() - 1) {
            // Compare decimal versions
            if bits_to_dec(&vec[i]) > bits_to_dec(&vec[i + 1]) {
                // let first = &vec[i].clone();
                // let second = &vec[i].clone();
                // vec.insert(i, second.to_string());
                // vec.insert(i + 1, first.to_string());
                vec.swap(i, i+1);
                sorted = false;
                converted += 1;
            }
        }
        // bar.inc(1);
        to_increment -= 1;
        // println!("Converted: {}", converted);
        if sorted == true {
            // bar.inc(to_increment);
            break;
        }
    }
    // for v in &tmp {
    //     if !vec.contains(v) {
    //         println!("{}", "Not equal!");
    //     }
    // }
    return vec;
}
#[allow(dead_code)]
fn bits_to_dec(str: &str) -> i32 {
    let mut dec = 0;
    for i in 0..str.len() {
        let s = &str[str.len()-i-1..str.len()-i];
        let s: i32 = s.parse::<i32>().unwrap();
        let s: i32 = s * 2_i32.pow(i as u32);
        dec += s;
    }
    dec
}
// #[allow(dead_code)]
// fn determine_most_common(arr: &Vec<String>, index: usize) -> i32 {
//     let mut ones = 0;
//     for i in 0..arr.len() {
//         if &arr[i][index..index+1] == "1" {
//             ones += 1;
//         }
//     }
//
//     return if ones >= 1000 / 2 {
//         1
//     } else {
//         0
//     }
// }
#[allow(dead_code)]
fn get_input() -> Vec<String> {
    let json = json_reader::read_json("src/inputs/input_day3.json");
    let json = json["bin"].clone();
    let json = JsonParser{ json };
    let arr = json.to_arr();
    let mut vec: Vec<String> = vec![];
    for i in 0..arr.len() {
        // println!("{}", arr[i].to_string());
        vec.push(arr[i].to_string());
        // println!("{:?}", vec)
    }
    return vec;
}
//
#[allow(dead_code)]
fn part1() {
    let arr = get_input();
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for j in 0..arr[0].len() {
        let mut one = 0;
        for i in 0..arr.len() {
            if &arr[i][j..j+1] == "1" {
                one += 1;
            }
        }
        if one >= arr.len() / 2 {
            gamma = format!("{}{}", gamma, "1");
            epsilon = format!("{}{}", epsilon, "0");
        } else {
            gamma = format!("{}{}", gamma, "0");
            epsilon = format!("{}{}", epsilon, "1");
        }
    }
    println!("gamma: {}", gamma);
    let mut gamma_dec: i32 = 0;
    let mut epsilon_dec: i32 = 0;
    println!("The binaries are:\ngamma: {}, epsilon: {}", gamma, epsilon);
    // To decimal
    for i in 0..gamma.len() {
        let g = &gamma[gamma.len()-i-1..gamma.len()-i];
        let e = &epsilon[epsilon.len()-i-1..epsilon.len()-i];
        let g: i32 = g.parse::<i32>().unwrap();
        let e: i32 = e.parse::<i32>().unwrap();
        let g: i32 = g * 2_i32.pow(i as u32);
        let e: i32 = e * 2_i32.pow(i as u32);
        gamma_dec += g;
        epsilon_dec += e;
    }
    println!("The power consumption is {}.", gamma_dec * epsilon_dec);
}
#[derive(Debug)]
struct JsonParser {
    json: JsonValue
}
impl JsonParser {
    pub fn to_arr(&self) -> Box<[&str]> {
        let mut vec: Vec<&str> = Vec::new();
        for i in 0..self.json.len() {
            let val: &str = self.json[i].as_str().unwrap_or("ERROR");
            vec.push(val);
        }
        vec.into_boxed_slice()
    }
}