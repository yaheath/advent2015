use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_input;

fn increment(pass: &mut [char] ) {
    let mut i = pass.len();
    let mut stop = false;
    while i > 0 && !stop {
        i -= 1;
        let c = match pass[i] {
            'a'..='y' => {
                stop = true;
                char::from_u32((pass[i] as u32) + 1).unwrap()
            },
            'z' => 'a',
            x => x,
        };
        pass[i] = c;
    }
}

fn is_valid(pass: &[char]) -> bool {
    let mut ok = false;
    for (&c1, &c2, &c3) in pass.iter().tuple_windows() {
        if (c1 as u32) + 1 == (c2 as u32) && (c2 as u32) + 1 == (c3 as u32) {
            ok = true;
            break;
        }
    }
    if !ok { return false; }
    ok = !pass.iter().any(|&c| c == 'i' || c == 'o' || c == 'l');
    if !ok { return false; }
    let mut pairs: Vec<usize> = Vec::new();
    for (idx, (&c1, &c2)) in pass.iter().tuple_windows::<(_,_)>().enumerate() {
        if c1 == c2 {
            pairs.push(idx);
        }
    }
    pairs.len() > 2 || (
        pairs.len() == 2 && pairs[0] + 1 != pairs[1]
    )
}

fn part1(input: &str) -> String {
    let mut pass = input.chars().collect::<Vec<_>>();
    increment(&mut pass);
    while !is_valid(&pass) {
        increment(&mut pass);
    }
    pass.iter().collect()
}

fn main() {
    let input: Vec<String> = read_input();
    let p = part1(&input[0]);
    println!("Part 1: {p}");
    println!("Part 2: {}", part1(&p));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_test() {
        let mut s = "abcdefgh".chars().collect::<Vec<_>>();
        increment(&mut s);
        assert_eq!(s, "abcdefgi".chars().collect::<Vec<_>>());
        s = "abcdefgz".chars().collect::<Vec<_>>();
        increment(&mut s);
        assert_eq!(s, "abcdefha".chars().collect::<Vec<_>>());
        s = "abcdefzz".chars().collect::<Vec<_>>();
        increment(&mut s);
        assert_eq!(s, "abcdegaa".chars().collect::<Vec<_>>());
        s = "abcdzzef".chars().collect::<Vec<_>>();
        increment(&mut s);
        assert_eq!(s, "abcdzzeg".chars().collect::<Vec<_>>());

        assert!(!is_valid(&("hijklmmn".chars().collect::<Vec<_>>())));
        assert!(!is_valid(&("abbceffg".chars().collect::<Vec<_>>())));
        assert!(!is_valid(&("abbcegjk".chars().collect::<Vec<_>>())));
        assert!(is_valid(&("abcdffaa".chars().collect::<Vec<_>>())));
        assert!(!is_valid(&("abcdfaaa".chars().collect::<Vec<_>>())));
        assert!(is_valid(&("ghjaabcc".chars().collect::<Vec<_>>())));

        assert_eq!(part1("abcdefgh"), "abcdffaa");
        assert_eq!(part1("ghijklmn"), "ghjaabcc");
//        assert_eq!(part2(&input), 0);
    }
}
