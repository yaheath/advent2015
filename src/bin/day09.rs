use std::collections::HashSet;
use std::str::FromStr;
use std::vec::Vec;
use advent_lib::read::read_input;

struct Input {
    a: String,
    b: String,
    dx: usize,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let w = s.split(' ').collect::<Vec<_>>();
        let dx = w[4].parse::<usize>().unwrap();
        Ok(Input{a: w[0].into(), b: w[2].into(), dx})
    }
}

fn brute_search<F>(
    from: String,
    dists: &Vec<Input>,
    sites: &HashSet<String>,
    visited: HashSet<String>,
    comp: &F,
) -> usize
where F: Fn(usize, usize) -> usize
{
    let mut ans: usize = 0;
    for s in sites.iter().filter(|x| !visited.contains(*x)) {
        if let Some(edge) = dists.iter()
                .find(|d| d.a == from && d.b == *s || d.b == from && d.a == *s) {
            let mut v = visited.clone();
            v.insert(s.clone());
            let mut n = edge.dx;
            if v.len() < sites.len() {
                n += brute_search(s.clone(), dists, sites, v, comp);
            }
            ans = if ans == 0 { n } else { comp(n, ans) }
        }
    }
    ans
}

fn doit(input: &Vec<Input>, ispart2: bool) -> usize {
    let mut sites: HashSet<String> = HashSet::new();
    for i in input.iter() {
        sites.insert(i.a.clone());
        sites.insert(i.b.clone());
    }
    let f = if ispart2 { |a:usize, b:usize| a.max(b) } else { |a:usize, b:usize| a.min(b) };
    sites.iter()
        .map(|s| brute_search(s.clone(), input, &sites, HashSet::from([s.clone()]), &f))
        .reduce(|acc, v| f(v, acc))
        .unwrap()
}

fn part1(input: &Vec<Input>) -> usize {
    doit(input, false)
}

fn part2(input: &Vec<Input>) -> usize {
    doit(input, true)
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day09_test() {
        let input: Vec<Input> = test_input(include_str!("day09.testinput"));
        assert_eq!(part1(&input), 605);
        assert_eq!(part2(&input), 982);
    }
}
