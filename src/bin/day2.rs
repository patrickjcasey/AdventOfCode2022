use std::fs;
use std::str;


fn part_one(contents: &str) -> i32 {
   let mut score = 0;
    for line in contents.lines(){
        let tokens:Vec<&str> = line.split(" ").collect();
        println!("{:?}", tokens);
        if tokens.len() < 2 {
            return score;
        }
        let p1 = tokens[0];
        let p2 = tokens[1];
        match p1 {
            "A" => match p2 {
                "X" => score += 4,
                "Y" => score += 8,
                "Z" => score += 3,
                _ => panic!("bad")
            }
            "B" => match p2 {
                "X" => score += 1,
                "Y" => score += 5,
                "Z" => score += 9,
                _ => panic!("bad")
            }
            "C" => match p2 {
                "X" => score += 7,
                "Y" => score += 2,
                "Z" => score += 6,
                _ => panic!("bad")
            }
            _ => panic!("bad")
        }
    }
    println!("{}", score);
    score
}

fn part_two(contents: &str) -> i32 {
    let mut score = 0;
    for line in contents.lines(){
        let tokens:Vec<&str> = line.split(" ").collect();
        println!("{:?}", tokens);
        if tokens.len() < 2 {
            return score;
        }
        let p1 = tokens[0];
        let p2 = tokens[1];
        match p1 {
            "A" => match p2 {
                "X" => score += 3,
                "Y" => score += 4,
                "Z" => score += 8,
                _ => panic!("bad")
            }
            "B" => match p2 {
                "X" => score += 1,
                "Y" => score += 5,
                "Z" => score += 9,
                _ => panic!("bad")
            }
            "C" => match p2 {
                "X" => score += 2,
                "Y" => score += 6,
                "Z" => score += 7,
                _ => panic!("bad")
            }
            _ => panic!("bad")
        }
    }
    println!("{}", score);
    score
}


fn main() {
    let contents = fs::read_to_string("./day2.txt").unwrap();
    println!("Part One: {}", part_one(&contents));
    println!("Part Two: {}", part_two(&contents));
}
