use std::collections::HashMap;
use std::vec::Vec;
use lazy_static::lazy_static;
use itertools::Itertools;
use regex::Regex;
use ya_advent_lib::read::read_input;

lazy_static! {
  static ref BAD_RE: Regex = Regex::new(
      r"(?:ab)|(?:cd)|(?:pq)|(?:xy)"
  ).unwrap();
}

fn is_nice1(s: &str) -> bool {
    if BAD_RE.is_match(s) {
        return false;
    }
    let mut vowels = 0;
    let mut haspair = false;
    for (c1, c2) in s.chars().chain(['.']).tuple_windows() {
        match c1 {
            'a' | 'e' | 'i' | 'o' | 'u' => {vowels += 1;},
            _ => {},
        }
        if c1 == c2 {
            haspair = true;
        }
    }
    haspair && vowels >= 3
}

fn is_nice2(s: &str) -> bool {
    let mut have_triple = false;
    for (c1, _, c3) in s.chars().tuple_windows() {
        if c1 == c3 {
            have_triple = true;
            break;
        }
    }
    if !have_triple { return false; }
    let mut pairs: HashMap<(char, char), usize> = HashMap::new();
    for (idx, (c1, c2)) in s.chars().tuple_windows::<(_,_)>().enumerate() {
        if let Some(val) = pairs.get(&(c1, c2)) {
            if val + 1 != idx {
                return true;
            }
        }
        else {
            pairs.insert((c1, c2), idx);
        }
    }
    false
}

fn part1(input: &[String]) -> usize {
    input.iter()
        .filter(|s| is_nice1(s))
        .count()
}

fn part2(input: &[String]) -> usize {
    input.iter()
        .filter(|s| is_nice2(s))
        .count()
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05_test() {
        assert!(is_nice1("ugknbfddgicrmopn"));
        assert!(is_nice1("aaa"));
        assert!(!is_nice1("jchzalrnumimnmhp"));
        assert!(!is_nice1("haegwjzuvuyypxyu"));
        assert!(!is_nice1("dvszwmarrgswjxmb"));

        assert!(is_nice2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice2("xxyxx"));
        assert!(!is_nice2("uurcxstgmygtbstg"));
        assert!(!is_nice2("ieodomkazucvgmuy"));
    }
}
