use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

#[derive(Debug)]
struct Input {
    //name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let w = s.split(' ').collect::<Vec<_>>();
        //let name = w[0].into();
        let capacity = w[2][0..w[2].len()-1].parse::<i64>().unwrap();
        let durability = w[4][0..w[4].len()-1].parse::<i64>().unwrap();
        let flavor = w[6][0..w[6].len()-1].parse::<i64>().unwrap();
        let texture = w[8][0..w[8].len()-1].parse::<i64>().unwrap();
        let calories = w[10].parse::<i64>().unwrap();
        Ok(Input{
            //name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        })
    }
}

fn mixtures(num_ingredients: usize, total_amt: i64) -> Vec<Vec<i64>> {
    let start = if num_ingredients == 1 { total_amt } else { 0 };
    let mut out: Vec<Vec<i64>> = Vec::new();
    for n in start..=total_amt {
        let remain = total_amt - n;
        if num_ingredients > 1 {
            for mut sub in mixtures(num_ingredients - 1, remain) {
                sub.push(n);
                out.push(sub);
            }
        }
        else {
            out.push(vec![n]);
        }
    }
    out
}

fn optimize(input: &Vec<Input>, ispart2: bool) -> i64 {
    let m = mixtures(input.len(), 100);
    //println!("{m:?}");
    m.iter()
        .flat_map(|v| {
            v.iter()
                .zip(input.iter())
                .map(|(amt, i)| (
                    amt * i.capacity,
                    amt * i.durability,
                    amt * i.flavor,
                    amt * i.texture,
                    amt * i.calories,
                ))
                .reduce(|acc, row| (
                    acc.0 + row.0,
                    acc.1 + row.1,
                    acc.2 + row.2,
                    acc.3 + row.3,
                    acc.4 + row.4,
                ))
                .filter(|(_,_,_,_,e)| !ispart2 || *e == 500)
                .map(|(a,b,c,d,_)| a.max(0)*b.max(0)*c.max(0)*d.max(0))
        })
        .max()
        .unwrap()
}

fn part1(input: &Vec<Input>) -> i64 {
    optimize(input, false)
}

fn part2(input: &Vec<Input>) -> i64 {
    optimize(input, true)
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
    fn day15_test() {
        let input: Vec<Input> = test_input(include_str!("day15.testinput"));
        assert_eq!(part1(&input), 62842880);
        assert_eq!(part2(&input), 57600000);
    }
}
