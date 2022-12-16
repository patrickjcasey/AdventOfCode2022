use std::collections::HashSet;
use std::fs;
use std::str;


fn get_priority(character: &char) -> u32 {
    if character.is_uppercase(){
        *character as u32 - 64 + 26
    }
    else{
        *character as u32 - 96
    }
}

fn get_unique_characters(half: &str) -> HashSet<u32> {
    let mut set = HashSet::new();
    for c in half.chars() {
        set.insert(get_priority(&c));
    }
    set
}


fn part_one(contents: &str) -> u32 {
    let mut score = 0;
    for line in contents.lines(){
        let (first_half, second_half) = line.split_at(line.len() / 2);
        let first_half_uniq_chars = get_unique_characters(first_half);
        let second_half_uniq_chars = get_unique_characters(second_half);
        let unique_priority = second_half_uniq_chars.intersection(&first_half_uniq_chars).next().unwrap();
        score += unique_priority;
    }
    score
}

fn part_two(contents: &str) -> u32 {
    let mut score = 0;
    let mut three_lines = vec!();
    for line in contents.lines(){
        three_lines.push(get_unique_characters(line));
        if three_lines.len() == 3 {
            let common = three_lines[0].intersection(&three_lines[1]).find(|x| three_lines[2].contains(x)); 
            match common {
                Some(x) => {
                    score += x;
                    three_lines = vec!();
                },
                None => {
                    panic!("BAD");
                },
            }            
        }
    }
    score
}


fn main() {
    let contents = fs::read_to_string("./day3.txt").unwrap();
    println!("Part One: {}", part_one(&contents));
    println!("Part Two: {}", part_two(&contents));
}
