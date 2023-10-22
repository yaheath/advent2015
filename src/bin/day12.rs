use std::vec::Vec;
use advent_lib::read::read_input;
use json::{parse, JsonValue};

fn sumnums(json: &JsonValue, ispart2: bool) -> f64 {
    match json {
        JsonValue::Object(o) => {
            if ispart2 && o.iter().find(|(_, &ref v)| {
                match v {
                    JsonValue::String(s) if s == "red" => true,
                    JsonValue::Short(s) if s == "red" => true,
                    _ => false,
                }
            }).is_some() {
                0.0
            } else {
                o.iter().map(|(_, v)| sumnums(v, ispart2)).sum()
            }
        },
        JsonValue::Array(a) => a.iter().map(|v| sumnums(v, ispart2)).sum(),
        JsonValue::Number(n) => (*n).into(),
        _ => 0.0,
    }
}

fn part1(input: &str) -> f64 {
    let json = parse(input).unwrap();
    sumnums(&json, false)
}

fn part2(input: &str) -> f64 {
    let json = parse(input).unwrap();
    sumnums(&json, true)
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
    fn day12_test() {
        assert_eq!(part1("[1,2,3]"), 6.0);
        assert_eq!(part1("{\"a\":2,\"b\":4}"), 6.0);
        assert_eq!(part1("[-1,{\"a\":1}]"), 0.0);
        assert_eq!(part1("[]"), 0.0);

        assert_eq!(part2("[1,2,3]"), 6.0);
        assert_eq!(part2("[1,{\"c\":\"red\",\"b\":2},3]"), 4.0);
        assert_eq!(part2("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"), 0.0);
    }
}
