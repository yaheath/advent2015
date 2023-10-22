use std::vec::Vec;
use advent_lib::read::read_input;

fn expand(s: String) -> String {
    let mut itr = s.chars();
    let mut current = String::new();
    let mut out: String = String::new();
    while let Some(c) = itr.next() {
        if current.len() == 0 || current.chars().next().unwrap() == c {
            current.push(c);
        }
        else {
            out += &format!("{}{}", current.len(), current.chars().next().unwrap());
            current = String::from(c);
        }
    }
    out += &format!("{}{}", current.len(), current.chars().next().unwrap());
    out
}

fn part1(input: &str) -> usize {
    let mut s = input.into();
    for _ in 0..40 {
        s = expand(s)
    }
    s.len()
}

fn part2(input: &str) -> usize {
    let mut s = input.into();
    for _ in 0..50 {
        s = expand(s)
    }
    s.len()
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input[0]));
    println!("Part 2: {}", part2(&input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_test() {
        assert_eq!(expand("1".into()), "11");
        assert_eq!(expand("11".into()), "21");
        assert_eq!(expand("21".into()), "1211");
        assert_eq!(expand("1211".into()), "111221");
        assert_eq!(expand("111221".into()), "312211");
    }
}
