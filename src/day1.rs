fn part_2() {
    let vals = read_input();
    
    let mut sums: Vec<i32> = Vec::new();
    for i in 0..(2000-2) {
        let sum = &vals[i].as_i32().unwrap() + &vals[i+1].as_i32().unwrap() + &vals[i+2].as_i32().unwrap();
        
        sums.push(sum);
    }
    
    let mut amt_bigger = 0;
    for i in 0..(sums.len() - 1) {
        if sums[i + 1] > sums[i] {
            amt_bigger += 1;
        }
    }
    
    println!("There are {} sums that are larger than the previous sum.", amt_bigger);
}

#[allow(unused)]
fn part_1() {
    let vals = read_input();
    // println!("{}", parsed["vals"]);
    
    let mut amt_bigger: u32 = 0;
    for index in 0..2000 {
        if index != 0 && &vals[index].as_i32() > &vals[index - 1].as_i32() {
            amt_bigger += 1;
        }
    }
    
    println!("There are {} measurements that are larger than the previous measurement.", amt_bigger)
}

fn read_input() -> JsonValue {
    let file = "src/input.json";
    let parsed = json_reader::read_json(file);
    parsed["vals"].clone()
}