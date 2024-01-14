use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

#[derive(Debug)]
struct Sue {
    num: usize,
    has: HashMap<String, usize>,
}

impl FromStr for Sue {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let w = s.split(' ').collect::<Vec<_>>();
        let num = w[1][0..w[1].len()-1].parse::<usize>().unwrap();
        let mut has: HashMap<String, usize> = HashMap::new();
        let mut i = 2;
        while i < w.len() {
            let what = &w[i][0..w[i].len()-1];
            i += 1;
            let qty = w[i].trim_matches(',').parse::<usize>().unwrap();
            i += 1;
            has.insert(what.to_string(), qty);
        }
        Ok(Sue{num, has})
    }
}

fn analysis() -> HashMap::<String, usize> {
    HashMap::from([
        ("children".into(), 3),
        ("cats".into(), 7),
        ("samoyeds".into(), 2),
        ("pomeranians".into(), 3),
        ("akitas".into(), 0),
        ("vizslas".into(), 0),
        ("goldfish".into(), 5),
        ("trees".into(), 3),
        ("cars".into(), 2),
        ("perfumes".into(), 1),
    ])
}

fn part1(input: &Vec<Sue>) -> usize {
    let a = analysis();
    let candidates = input.iter()
        .filter(|s| s.has.iter()
            .all(|(item, qty)| a[item] == *qty)
        )
        .collect::<Vec<_>>();
    //println!("{candidates:?}");
    assert_eq!(candidates.len(), 1);
    candidates[0].num
}

fn part2(input: &Vec<Sue>) -> usize {
    let a = analysis();
    let candidates = input.iter()
        .filter(|s| s.has.iter()
            .all(|(item, qty)|
                if item == "cats" || item == "trees" {
                    a[item] < *qty
                }
                else if item == "pomeranians" || item == "goldfish" {
                    a[item] > *qty
                }
                else {
                    a[item] == *qty
                }
            )
        )
        .collect::<Vec<_>>();
    //println!("{candidates:?}");
    assert_eq!(candidates.len(), 1);
    candidates[0].num
}

fn main() {
    let input: Vec<Sue> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn dayNN_test() {
        let input: Vec<Sue> = test_input(include_str!("dayNN.testinput"));
        assert_eq!(part1(&input), 0);
        assert_eq!(part2(&input), 0);
    }
}
*/
