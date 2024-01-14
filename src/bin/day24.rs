use std::collections::HashSet;
use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_input;

fn part1(input: &Vec<u64>) -> u64 {
    let all: HashSet<u64> = HashSet::from_iter(input.iter().copied());
    let mut found = 0usize;
    let mut min_qe = u64::MAX;
    let tgt = all.iter().copied().sum::<u64>() / 3;
    for s in input.iter().copied().powerset().filter(|s| s.len() > 0) {
        if found > 0 && s.len() > found {
            return min_qe;
        }
        if s.len() == 0 {
            break;
        }
        let w: u64 = s.iter().copied().sum();
        if w != tgt { continue; }
        let qe: u64 = s.iter().copied().product();
        let set: HashSet<u64> = HashSet::from_iter(s.iter().copied());
        for s1 in all.difference(&set).copied().powerset().filter(|s| s.len() > 0) {
            if s1.iter().sum::<u64>() == w {
                if found == 0 {
                    found = s.len();
                }
                min_qe = min_qe.min(qe);
            }
        }
    }
    panic!();
}

fn part2(input: &Vec<u64>) -> u64 {
    let all: HashSet<u64> = HashSet::from_iter(input.iter().copied());
    let mut found = 0usize;
    let mut min_qe = u64::MAX;
    let tgt = all.iter().copied().sum::<u64>() / 4;
    for s in input.iter().copied().powerset().filter(|s| s.len() > 0) {
        if found > 0 && s.len() > found {
            return min_qe;
        }
        if s.len() == 0 {
            break;
        }
        let w: u64 = s.iter().copied().sum();
        if w != tgt { continue; }
        let qe: u64 = s.iter().copied().product();
        let set: HashSet<u64> = HashSet::from_iter(s.iter().copied());
        for s1 in all.difference(&set).copied().powerset().filter(|s| s.len() > 0) {
            if s1.iter().sum::<u64>() == w {
                let set1: HashSet<u64> = HashSet::from_iter(s1.iter().copied());
                for s2 in set.difference(&set1).copied().powerset().filter(|s| s.len() > 0) {
                    if s2.iter().sum::<u64>() == w {
                        if found == 0 {
                            found = s.len();
                        }
                        min_qe = min_qe.min(qe);
                    }
                }
            }
        }
    }
    panic!();
}

fn main() {
    let input: Vec<u64> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day24_test() {
        let input: Vec<u64> = test_input(include_str!("day24.testinput"));
        assert_eq!(part1(&input), 99);
        assert_eq!(part2(&input), 44);
    }
}
