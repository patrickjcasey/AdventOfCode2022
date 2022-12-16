use std::fs;
use std::str;


fn part_one(contents: &str) -> i32 {
    let mut max_calories:i32 = 0;
    let mut current_calories:i32 = 0;
    for line in contents.lines(){
        if line.len() == 0 {
            if current_calories > max_calories {
                max_calories = current_calories;      
            }
            current_calories = 0;
        }
        else {
            current_calories += line.parse::<i32>().unwrap();
        }
    }
    return max_calories;
}

fn part_two(contents: &str) -> i32 {
    let mut totals: Vec<i32> = vec!();
    let mut total = 0;
    for line in contents.lines() {
        if line.len() == 0 {
            totals.push(total);
            total = 0;
        }
        else{
            total += line.parse::<i32>().unwrap();
        }
    }
    totals.sort();
    return totals[totals.len()-1] + totals[totals.len()-2] + totals[totals.len()-3];
}


fn main() {
    let contents = fs::read_to_string("./day1.txt").unwrap();
    println!("Part One: {}", part_one(&contents));
    println!("Part Two: {}", part_two(&contents));
}
