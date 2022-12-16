use std::fs;
use std::str;

fn find_range(range_str: &str) -> Vec<u32> {
    match range_str.split_once("-") {
        Some((low, high)) => {
            (low.parse::<u32>().unwrap()..high.parse::<u32>().unwrap() + 1).collect()
        },
        None => panic!("BAD RANGE"),
    }
}

fn is_subset(list1: Vec<u32>, list2: Vec<u32>) -> bool {
    list1.iter().all(|x| list2.contains(x)) || list2.iter().all(|x| list1.contains(x))
}

fn overlap_at_all(list1: Vec<u32>, list2: Vec<u32>) -> bool {
    list1.iter().any(|x| list2.contains(x))
}

fn part_one(contents: &str) -> u32 {
    let mut inclusive_pairs = 0;
    for line in contents.lines(){
        println!("{}", line);
        match line.split_once(",") {
            Some((first_half, second_half)) => {
                let range1 = find_range(first_half);
                let range2 = find_range(second_half);
                if is_subset(range1, range2) {
                    inclusive_pairs += 1
                }
            }
            None => {
               println!("skip"); 
            }
        }
    }
    inclusive_pairs
}

fn part_two(contents: &str) -> u32 {
    let mut inclusive_pairs = 0;
    for line in contents.lines(){
        println!("{}", line);
        match line.split_once(",") {
            Some((first_half, second_half)) => {
                let range1 = find_range(first_half);
                let range2 = find_range(second_half);
                if overlap_at_all(range1, range2) {
                    inclusive_pairs += 1
                }
            }
            None => {
               println!("skip"); 
            }
        }
    }
    inclusive_pairs

}


fn main() {
    let contents = fs::read_to_string("./day4.txt").unwrap();
    println!("Part One: {}", part_one(&contents));
    println!("Part Two: {}", part_two(&contents));
}
