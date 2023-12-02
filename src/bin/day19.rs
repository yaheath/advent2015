use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use rand::thread_rng;
use rand::seq::SliceRandom;
use advent_lib::read::read_sectioned_input;

#[derive(Clone)]
struct Rep {
    from: String,
    to: String,
}

impl FromStr for Rep {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let w = s.split(' ').collect::<Vec<_>>();
        let from = w[0].into();
        let to = w[2].into();
        Ok(Rep{from, to})
    }
}

type Input = (Vec<Rep>, Vec<String>);

fn part1(input: &Input) -> usize {
    let base = &input.1[0];
    let reps = &input.0;
    let blen = base.len();
    let mut result: Vec<String> = Vec::new();
    for i in 0..blen {
        let pre = &base[0..i];
        let post = &base[i..];
        for r in reps.iter() {
            if post.starts_with(&r.from) {
                let mut s = String::from(pre);
                s += &post.replacen(&r.from, &r.to, 1);
                result.push(s);
            }
        }
    }
    result.sort();
    result.into_iter().dedup().count()
}

fn part2(input: &Input) -> usize {
    let mut mol = input.1[0].clone();
    'outer: loop {
        let mut rreps = input.0.clone();
        rreps.shuffle(&mut thread_rng());
        let mut steps = 0;
        while mol != "e" {
            let mut ok = false;
            for r in rreps.iter() {
                if r.from == "e" {
                    if mol == r.to {
                        mol = String::from("e");
                        ok = true;
                        steps += 1;
                        break;
                    }
                } else if mol.contains(&r.to) {
                    mol = mol.replacen(&r.to, &r.from, 1);
                    ok = true;
                    steps += 1;
                    break;
                }
            }
            if !ok {
                continue 'outer;
            }
        }
        return steps;
    }
}

fn main() {
    let input: Input = read_sectioned_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::sectioned_test_input;

    #[test]
    fn day19_test() {
        let input: Input = sectioned_test_input(include_str!("day19.testinput"));
        assert_eq!(part1(&input), 7);
        assert_eq!(part2(&input), 6);
    }
}
