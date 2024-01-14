use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

struct Gift {
    l: usize,
    w: usize,
    h: usize,
}

impl Gift {
    fn wp_area(&self) -> usize {
        let s1 = self.l * self.w;
        let s2 = self.h * self.w;
        let s3 = self.h * self.l;
        s1 * 2 + s2 * 2 + s3 * 2 + s1.min(s2).min(s3)
    }

    fn ribbon(&self) -> usize {
        self.l * 2 + self.w * 2 + self.l * self.h * self.w
    }
}

impl FromStr for Gift {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut d = s.split('x').map(|ss| ss.parse::<usize>().unwrap()).collect::<Vec<_>>();
        d.sort();
        Ok(Gift{l: d[0], w: d[1], h: d[2]})
    }
}

fn part1(input: &[Gift]) -> usize {
    input.iter().fold(0, |a, g| a + g.wp_area())
}

fn part2(input: &[Gift]) -> usize {
    input.iter().fold(0, |r, g| r + g.ribbon())
}

fn main() {
    let input: Vec<Gift> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day02() {
        let input: Vec<Gift> = test_input("2x3x4\n1x1x10\n");
        assert_eq!(part1(&input), 58 + 43);
        assert_eq!(part2(&input), 34 + 14);
    }
}
