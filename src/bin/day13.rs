use std::collections::HashSet;
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_input;

struct Input {
    name: String,
    neighbor: String,
    change: i64,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split(' ').collect::<Vec<_>>();
        let name = v[0].into();
        let neighbor = v[10][0..v[10].len() - 1].into();
        let change = v[3].parse::<i64>().unwrap() *
            if v[2] == "gain" { 1 } else { -1 };
        Ok(Input{name, neighbor, change})
    }
}

fn find_dx(input: &Vec<Input>, a: &String, b: &String) -> i64 {
    if let Some(m) = input.iter()
        .find(|&i| i.name == *a && i.neighbor == *b) {
            m.change
    } else {
        0
    }
}

fn calc(input: &Vec<Input>, first: &String) -> i64 {
    let mut names: HashSet<String> = input.iter()
        .flat_map(|i| vec![i.name.clone(), i.neighbor.clone()])
        .collect();
    names.remove(first);
    names.iter()
        .permutations(names.len())
        .map(|list| {
            let mut v = list.clone();
            v.push(&first);
            v.iter()
                .circular_tuple_windows()
                .fold(0, |acc, (&a, &b)| acc + find_dx(input, a, b) + find_dx(input, b, a))
        })
        .max()
        .unwrap()
}

fn part1(input: &Vec<Input>) -> i64 {
    calc(input, &input[0].name)
}

fn part2(input: &Vec<Input>) -> i64 {
    calc(input, &String::from("me"))
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
    fn day13_test() {
        let input: Vec<Input> = test_input(include_str!("day13.testinput"));
        assert_eq!(part1(&input), 330);
        assert_eq!(part2(&input), 286);
    }
}
