use std::vec::Vec;
use ya_advent_lib::read::read_input;

fn expand(s: &str) -> String {
    let itr = s.chars();
    let mut current = String::new();
    let mut out: String = String::new();
    for c in itr {
        if current.is_empty() || current.chars().next().unwrap() == c {
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
    let mut s: String = input.into();
    for _ in 0..40 {
        s = expand(&s)
    }
    s.len()
}

fn part2(input: &str) -> usize {
    let mut s: String = input.into();
    for _ in 0..50 {
        s = expand(&s)
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
        assert_eq!(expand("1"), "11");
        assert_eq!(expand("11"), "21");
        assert_eq!(expand("21"), "1211");
        assert_eq!(expand("1211"), "111221");
        assert_eq!(expand("111221"), "312211");
    }
}
