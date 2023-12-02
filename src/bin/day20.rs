use std::vec::Vec;
use advent_lib::read::read_input;

fn presents(housenum: usize) -> usize {
    let mut sum = 0;
    let d = if housenum < 1000 {
        housenum
    } else {
        (housenum as f64).sqrt() as usize + 1
    };
    for i in 1..=d {
        if housenum % i == 0 {
            sum += i;
            if housenum >= 1000 {
                sum += housenum / i;
            }
        }
    }
    sum * 10
}

fn presents2(housenum: usize) -> usize {
    let mut sum = 0;
    let d = (housenum as f64).sqrt() as usize + 1;
    for i in 1..=d {
        if housenum % i == 0 {
            if i <= 50 {
                sum += housenum/i;
            }
            if housenum/i <= 50 {
                sum += i;
            }
        }
    }
    sum * 11
}

fn part1(input: usize) -> usize {
    for n in 1.. {
        let p = presents(n);
        if p >= input { return n; }
    }
    panic!()
}

fn part2(input: usize) -> usize {
    for n in 1.. {
        let p = presents2(n);
        if p >= input { return n; }
    }
    panic!()
}

fn main() {
    let input: Vec<usize> = read_input();
    println!("Part 1: {}", part1(input[0]));
    println!("Part 2: {}", part2(input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day20_test() {
        assert_eq!(presents(1), 10);
        assert_eq!(presents(2), 30);
        assert_eq!(presents(3), 40);
        assert_eq!(presents(6), 120);
        assert_eq!(presents(9), 130);
        assert_eq!(part1(150), 8);
    }
}
