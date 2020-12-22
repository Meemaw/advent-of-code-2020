use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_input() -> Vec<Vec<char>> {
    let f = File::open("src/input.txt").expect("Failed to read input file!");
    return BufReader::new(f)
        .lines()
        .map(|x| x.unwrap())
        .map(|line| line.chars().collect())
        .collect();
}

struct SlopeConfiguration {
    jump_x: usize,
    jump_y: usize,
}

fn solve(grid: &Vec<Vec<char>>, slope_configuration: SlopeConfiguration) -> usize {
    let mut count: usize = 0;
    let mut x: usize = 0;
    let mut y: usize = 0;
    let height = grid.len();
    let width = grid[0].len();

    while y < height - 1 {
        x += slope_configuration.jump_x;
        y += slope_configuration.jump_y;
        if grid[y][x % width] == '#' {
            count += 1;
        }
    }

    return count;
}

fn part1() -> usize {
    return solve(
        &read_input(),
        SlopeConfiguration {
            jump_x: 3,
            jump_y: 1,
        },
    );
}

fn part2() -> usize {
    let grid = read_input();
    return [
        solve(
            &grid,
            SlopeConfiguration {
                jump_x: 1,
                jump_y: 1,
            },
        ),
        solve(
            &grid,
            SlopeConfiguration {
                jump_x: 3,
                jump_y: 1,
            },
        ),
        solve(
            &grid,
            SlopeConfiguration {
                jump_x: 5,
                jump_y: 1,
            },
        ),
        solve(
            &grid,
            SlopeConfiguration {
                jump_x: 7,
                jump_y: 1,
            },
        ),
        solve(
            &grid,
            SlopeConfiguration {
                jump_x: 1,
                jump_y: 2,
            },
        ),
    ]
    .iter()
    .fold(1, |acc, x| acc * x);
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
        assert_eq!(part1(), 237);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2106818610);
    }
}
