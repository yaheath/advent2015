use std::vec::Vec;
use advent_lib::read::read_input;

fn countchars(s: &str) -> (usize, usize) {
    let mut itr = s.chars();
    let repr = s.len();
    let mut mem = 0;
    while let Some(c) = itr.next() {
        match c {
            '"' => {},
            '\\' => {
                let n = itr.next().unwrap();
                match n {
                    '"' | '\\' => {},
                    'x' => {
                        itr.next();
                        itr.next();
                    },
                    _ => panic!("illegal escape"),
                }
                mem += 1;
            }
            _ => { mem += 1; }
        }
    }
    (repr, mem)
}

fn escape(s: &str) -> String {
    let mut out = String::from('"');
    for c in s.chars() {
        match c {
            '"' | '\\' => { out.push('\\'); },
            _ => {},
        }
        out.push(c);
    }
    out.push('"');
    out
}

fn part1(input: &Vec<String>) -> usize {
    input.iter()
        .map(|s| {
            let (r, m) = countchars(s);
            r - m
        })
        .sum()
}

fn part2(input: &Vec<String>) -> usize {
    input.iter()
        .map(|s| escape(s).len() - s.len())
        .sum()

}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day08_test() {
        let input: Vec<String> = test_input(include_str!("day08.testinput"));
        assert_eq!(part1(&input), 12);
        assert_eq!(part2(&input), 19);
    }
}
