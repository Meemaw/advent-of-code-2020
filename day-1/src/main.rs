use core::panic;
use std::io::prelude::*;
use std::io::BufReader;
use std::{collections::HashSet, fs::File};

fn read_input() -> HashSet<i32> {
    let f = File::open("src/input.txt").expect("Failed to read input file!");
    return BufReader::new(f)
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();
}

fn part1() -> i32 {
    let values = read_input();
    for value in values.iter() {
        let target: i32 = 2020 - value;
        if values.contains(&target) {
            return target * value;
        }
    }
    panic!("Could not solve part 1");
}

fn part2() -> i32 {
    let values = read_input();
    for value in values.iter() {
        let target: i32 = 2020 - value;
        for another_value in values.iter() {
            if another_value != value {
                let another_target: i32 = target - another_value;
                if values.contains(&another_target) {
                    let last_value: i32 = 2020 - value - another_target;
                    return value * another_target * last_value;
                }
            }
        }
    }

    panic!("Could not solve part 2");
}

fn main() {
    println!("Result part 1: {}", part1());
    println!("Result part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 877971);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 203481432);
    }
}
