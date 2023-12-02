use std::str::FromStr;
use std::vec::Vec;
use lazy_static::lazy_static;
use regex::Regex;
use advent_lib::read::read_input;

struct Input {
    row: usize,
    col: usize,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"row (\d+), column (\d+)"
            ).unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let row:usize = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let col:usize = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            Ok(Input {row, col})
        }
        else {
            Err(())
        }
    }
}

fn get_n(row: usize, col: usize) -> usize {
    (1 .. row + col - 1).into_iter().sum::<usize>() + col
}

fn part1(input: &Input) -> usize {
    let mut code = 20151125;
    for _ in 1 .. get_n(input.row, input.col) {
        code = (code * 252533) % 33554393;
    }
    code
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day25_test() {
        assert_eq!(part1(&Input {row: 2, col: 3}), 16929656);
        assert_eq!(part1(&Input {row: 6, col: 6}), 27995004);
    }
}
