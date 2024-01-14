use std::vec::Vec;
use advent_lib::read::read_input;

fn mt_search(input: &str, goal: &str, start: usize) -> usize {
    for n in start.. {
        let key = format!("{input}{n}");
        let hash = format!("{:x}", md5::compute(key.as_bytes()));
        if hash.starts_with(goal) {
            return n;
        }
    }
    panic!();
}

fn part1(input: &str) -> usize {
    mt_search(input, "00000", 1)
}

fn part2(input: &str, start: usize) -> usize {
    mt_search(input, "000000", start)
}

fn main() {
    let input: Vec<String> = read_input();
    let p1 = part1(&input[0]);
    println!("Part 1: {p1}");
    println!("Part 2: {}", part2(&input[0], p1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day04_test() {
        assert_eq!(part1("abcdef"), 609043);
        assert_eq!(part1("pqrstuv"), 1048970);
    }
}
