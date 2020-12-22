use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct PasswordPolicy {
  i1: usize,
  i2: usize,
  value: char,
  password: String,
}

fn parse_password_policy(s: String) -> PasswordPolicy {
  let split: Vec<&str> = s.split(" ").collect();
  let indices: Vec<&str> = split[0].split("-").collect();
  return PasswordPolicy {
    password: String::from(split[2]),
    value: split[1].chars().next().unwrap(),
    i1: indices[0].parse().unwrap(),
    i2: indices[1].parse().unwrap(),
  };
}

fn part1() -> usize {
  let f = File::open("src/input.txt").expect("Failed to read input file!");
  return BufReader::new(f)
    .lines()
    .map(|x| x.unwrap())
    .map(|x| parse_password_policy(x))
    .filter(|policy| {
      let num_matching: usize = policy
        .password
        .chars()
        .filter(|c| *c == policy.value)
        .count();
      return num_matching >= policy.i1 && num_matching <= policy.i2;
    })
    .count();
}

fn part2() -> usize {
  let f = File::open("src/input.txt").expect("Failed to read input file!");
  return BufReader::new(f)
    .lines()
    .map(|x| x.unwrap())
    .map(|x| parse_password_policy(x))
    .filter(|policy| {
      return (policy.password.chars().nth(policy.i1 - 1).unwrap() == policy.value)
        ^ (policy.password.chars().nth(policy.i2 - 1).unwrap() == policy.value);
    })
    .count();
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
    assert_eq!(part1(), 515);
  }

  #[test]
  fn test_part2() {
    assert_eq!(part2(), 711);
  }
}
