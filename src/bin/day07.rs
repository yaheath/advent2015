use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use advent_lib::read::read_input;

#[derive(Clone)]
enum Input {
    AssignImmed(u16, String),
    Assign(String, String),
    AndImmed(u16, String, String),
    And(String, String, String),
    Or(String, String, String),
    LShift(String, u16, String),
    RShift(String, u16, String),
    Not(String, String),
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr = s.split(" -> ");
        let lhs = itr.next().ok_or(())?;
        let tgt = itr.next().ok_or(())?;
        let lhs = lhs.split(" ").collect::<Vec<_>>();
        if lhs.len() == 1 {
            if let Ok(v) = lhs[0].parse::<u16>() {
                Ok(Input::AssignImmed(v, tgt.into()))
            }
            else {
                Ok(Input::Assign(lhs[0].into(), tgt.into()))
            }
        }
        else if lhs.len() == 2 {
            Ok(Input::Not(lhs[1].into(), tgt.into()))
        }
        else if lhs.len() == 3 {
            match lhs[1] {
                "AND" => {
                    if let Ok(v) = lhs[0].parse::<u16>() {
                        Ok(Input::AndImmed(v, lhs[2].into(), tgt.into()))
                    }
                    else {
                        Ok(Input::And(lhs[0].into(), lhs[2].into(), tgt.into()))
                    }
                },
                "OR" => Ok(Input::Or(lhs[0].into(), lhs[2].into(), tgt.into())),
                "LSHIFT" =>
                    Ok(Input::LShift(lhs[0].into(), lhs[2].parse::<u16>().map_err(|_| ())?, tgt.into())),
                "RSHIFT" =>
                    Ok(Input::RShift(lhs[0].into(), lhs[2].parse::<u16>().map_err(|_| ())?, tgt.into())),
                _ => Err(()),
            }
        }
        else {
            Err(())
        }
    }
}

fn process(input: &Vec<Input>) -> HashMap<String, u16> {
    let mut wires:HashMap<String, u16> = HashMap::new();
    let mut remaining: Vec<Input> = input.clone();

    while remaining.len() > 0 {
        let rows = remaining.len();
        let mut next: Vec<Input> = Vec::new();
        for i in remaining.into_iter() {
            match i {
                Input::AssignImmed(n, ref tgt) => {
                    wires.insert(tgt.clone(), n);
                },
                Input::Assign(ref a, ref tgt) => {
                    if !wires.contains_key(a) {
                        next.push(i);
                    }
                    else {
                        wires.insert(tgt.clone(), wires[a]);
                    }
                },
                Input::AndImmed(n, ref b, ref tgt) => {
                    if !wires.contains_key(b) {
                        next.push(i);
                    }
                    else {
                        let val = n & wires[b];
                        wires.insert(tgt.clone(), val);
                    }
                },
                Input::And(ref a, ref b, ref tgt) => {
                    if !wires.contains_key(a) || !wires.contains_key(b) {
                        next.push(i);
                    }
                    else {
                        let val = wires[a] & wires[b];
                        wires.insert(tgt.clone(), val);
                    }
                },
                Input::Or(ref a, ref b, ref tgt) => {
                    if !wires.contains_key(a) || !wires.contains_key(b) {
                        next.push(i);
                    }
                    else {
                        let val = wires[a] | wires[b];
                        wires.insert(tgt.clone(), val);
                    }
                },
                Input::LShift(ref a, n, ref tgt) => {
                    if !wires.contains_key(a) {
                        next.push(i);
                    }
                    else {
                        let val = wires[a] << n;
                        wires.insert(tgt.clone(), val);
                    }
                },
                Input::RShift(ref a, n, ref tgt) => {
                    if !wires.contains_key(a) {
                        next.push(i);
                    }
                    else {
                        let val = wires[a] >> n;
                        wires.insert(tgt.clone(), val);
                    }
                },
                Input::Not(ref a, ref tgt) => {
                    if !wires.contains_key(a) {
                        next.push(i);
                    }
                    else {
                        let val = !wires[a];
                        wires.insert(tgt.clone(), val);
                    }
                },
            }
        }
        if next.len() == rows {
            panic!("no items resolved rows={rows}");
        }
        remaining = next;
    }

    wires
}

fn part1(input: &Vec<Input>) -> u16 {
    let wires = process(input);
    wires["a"]
}

fn part2(input: &Vec<Input>, a: u16) -> u16 {
    let updated = input.iter()
        .map(|i| match i {
            Input::AssignImmed(_, w) if (w == "b") => Input::AssignImmed(a, "b".into()),
            _ => i.clone(),
        })
        .collect();
    let wires = process(&updated);
    wires["a"]
}

fn main() {
    let input: Vec<Input> = read_input();
    let a = part1(&input);
    println!("Part 1: {a}");
    println!("Part 2: {}", part2(&input, a));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day07_test() {
        let input: Vec<Input> = test_input(include_str!("day07.testinput"));
        let wires = process(&input);
        assert_eq!(wires.get("d"), Some(&72));
        assert_eq!(wires.get("e"), Some(&507));
        assert_eq!(wires.get("f"), Some(&492));
        assert_eq!(wires.get("g"), Some(&114));
        assert_eq!(wires.get("h"), Some(&65412));
        assert_eq!(wires.get("i"), Some(&65079));
        assert_eq!(wires.get("x"), Some(&123));
        assert_eq!(wires.get("y"), Some(&456));
    }
}
