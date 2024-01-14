use std::cmp::Reverse;
use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_input;

struct Input {
    name: String,
    speed: i64,
    duration: i64,
    rest: i64,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let w = s.split(' ').collect::<Vec<_>>();
        let name = w[0].into();
        let speed = w[3].parse::<i64>().unwrap();
        let duration = w[6].parse::<i64>().unwrap();
        let rest = w[13].parse::<i64>().unwrap();
        Ok(Input{name, speed, duration, rest})
    }
}

fn dx_after(r: &Input, time: i64) -> i64 {
    let cycle = r.duration + r.rest;
    let ncycles = time / cycle;
    let rem = time % cycle;
    ncycles * (r.duration * r.speed) + rem.min(r.duration) * r.speed
}

fn calc(input: &Vec<Input>, time: i64) -> i64 {
    input.iter().map(|r| dx_after(r, time)).max().unwrap()
}

fn part1(input: &Vec<Input>) -> i64 {
    calc(input, 2503)
}

fn calc2(input: &Vec<Input>, time: i64) -> i64 {
    let mut scores: HashMap<&String, i64> = HashMap::new();
    for t in 1..time {
        let by_score = input.iter()
            .map(|r| (r, dx_after(r, t)))
            .sorted_unstable_by_key(|(_, d)| Reverse(*d))
            .collect::<Vec<_>>();
        let lead = by_score[0].1;
        for r in by_score.iter().filter(|(_, x)| *x == lead) {
            scores.entry(&r.0.name)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }
    scores.iter().map(|(_,&v)| v).max().unwrap()
}

fn part2(input: &Vec<Input>) -> i64 {
    calc2(input, 2503)
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day14_test() {
        let input: Vec<Input> = test_input(include_str!("day14.testinput"));
        assert_eq!(calc(&input, 1000), 1120);
        assert_eq!(calc2(&input, 1000), 689);
    }
}
