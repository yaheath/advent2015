use std::vec::Vec;
use advent_lib::read::read_input;

fn part1(input: &str) -> i64 {
    input.chars()
        .fold(0, |f, c| f + match c {
            '(' => 1,
            ')' => -1,
            _ => panic!(),
        })
}

fn part2(input: &str) -> i64 {
    let mut f = 0;
    for (idx, c) in input.chars().enumerate() {
        f += match c {
            '(' => 1,
            ')' => -1,
            _ => panic!(),
        };
        if f == -1 {
            return idx as i64 + 1;
        }
    }
    panic!();
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
    fn day01_test() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
        assert_eq!(part1("))((((("), 3);
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);

        assert_eq!(part2(")"), 1);
        assert_eq!(part2("()())"), 5);
    }
}
