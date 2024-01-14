use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_input;

fn fill(input: &[usize], total: usize) -> usize {
    input.iter()
        .powerset()
        .filter(|v| !v.is_empty())
        .filter(|v| v.iter().fold(0, |acc, n| acc + **n) == total)
        .count()
}

fn part1(input: &[usize]) -> usize {
    fill(input, 150)
}

fn fill2(input: &[usize], total: usize) -> usize {
    input.iter()
        .powerset()
        .filter(|v| !v.is_empty())
        .filter(|v| v.iter().fold(0, |acc, n| acc + **n) == total)
        .sorted_by_key(|v| v.len())
        .fold((0, 0), |acc, v|
            if acc.0 == 0 {
                (1, v.len())
            } else if v.len() == acc.1 {
                (acc.0 + 1, acc.1)
            } else {
                acc
            }
        )
        .0
}

fn part2(input: &[usize]) -> usize {
    fill2(input, 150)
}

fn main() {
    let input: Vec<usize> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day17_test() {
        let input: Vec<usize> = test_input("20\n15\n10\n5\n5\n");
        assert_eq!(fill(&input, 25), 4);
        assert_eq!(fill2(&input, 25), 3);
    }
}
