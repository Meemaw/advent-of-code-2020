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
  let i1: usize = indices[0].parse().unwrap();
  let i2: usize = indices[1].parse().unwrap();
  let value = split[1].chars().next().unwrap();
  let password = split[2];
  return PasswordPolicy {
    password: String::from(password),
    value,
    i1,
    i2,
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
