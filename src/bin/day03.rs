use std::collections::HashSet;
use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_input;
use ya_advent_lib::coords::{CDir, Coord2D};

fn str_to_dirs(input: &str) -> impl Iterator<Item=CDir> + '_ {
    input.chars().map(|c| match c {
        '^' => CDir::N,
        'v' => CDir::S,
        '>' => CDir::E,
        '<' => CDir::W,
        _ => panic!(),
    })
}

fn part1(input: &str) -> usize {
    let mut set: HashSet<Coord2D> = HashSet::new();
    let mut pos = Coord2D::new(0, 0);
    set.insert(pos);
    for d in str_to_dirs(input) {
        pos += d;
        set.insert(pos);
    }
    set.len()
}

fn part2(input: &str) -> usize {
    let mut set: HashSet<Coord2D> = HashSet::new();
    let mut pos1 = Coord2D::new(0, 0);
    let mut pos2 = Coord2D::new(0, 0);
    set.insert(pos1);
    for chunk in &str_to_dirs(input).chunks(2) {
        let pair = chunk.collect::<Vec<_>>();
        pos1 += pair[0];
        set.insert(pos1);
        if pair.len() > 1 {
            pos2 += pair[1];
            set.insert(pos2);
        }
    }
    set.len()
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
    fn day03_test() {
        assert_eq!(part1(">"), 2);
        assert_eq!(part1("^>v<"), 4);
        assert_eq!(part1("^v^v^v^v^v"), 2);
        assert_eq!(part2("^v"), 3);
        assert_eq!(part2("^>v<"), 3);
        assert_eq!(part2("^v^v^v^v^v"), 11);
    }
}
