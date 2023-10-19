use std::str::FromStr;
use std::vec::Vec;
use advent_lib::read::read_input;
use advent_lib::coords::Coord2D;
use advent_lib::grid::Grid;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Input {
    corner1: Coord2D,
    corner2: Coord2D,
    action: Action,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr = s.split(' ');
        let mut action = itr.next().ok_or(())?;
        if action == "turn" {
            action = itr.next().ok_or(())?;
        }
        let action = match action {
            "on" => Ok(Action::TurnOn),
            "off" => Ok(Action::TurnOff),
            "toggle" => Ok(Action::Toggle),
            _ => Err(()),
        }?;

        let corner1 = itr.next().ok_or(()).map(|s| s.parse::<Coord2D>())??;
        itr.next().ok_or(())?;
        let corner2 = itr.next().ok_or(()).map(|s| s.parse::<Coord2D>())??;

        Ok(Input{corner1, corner2, action})
    }
}

fn part1(input: &Vec<Input>) -> usize {
    let mut grid: Grid<bool> = Grid::new(0, 0, 999, 999, false);
    for inst in input {
        for y in inst.corner1.y ..= inst.corner2.y {
            for x in inst.corner1.x ..= inst.corner2.x {
                let val = match inst.action {
                    Action::TurnOn => true,
                    Action::TurnOff => false,
                    Action::Toggle => !grid.get(x, y),
                };
                grid.set(x, y, val);
            }
        }
    }
    grid.iter().filter(|v| **v).count()
}

fn part2(input: &Vec<Input>) -> usize {
    let mut grid: Grid<usize> = Grid::new(0, 0, 999, 999, 0);
    for inst in input {
        for y in inst.corner1.y ..= inst.corner2.y {
            for x in inst.corner1.x ..= inst.corner2.x {
                let val = grid.get(x, y);
                let val = match inst.action {
                    Action::TurnOn => val + 1,
                    Action::TurnOff => val.saturating_sub(1),
                    Action::Toggle => val + 2,
                };
                grid.set(x, y, val);
            }
        }
    }
    grid.iter().sum()
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
    fn day06_test() {
        let input: Vec<Input> = test_input("turn on 0,0 through 999,999");
        assert_eq!(part1(&input), 1000000);
        let input: Vec<Input> = test_input("toggle 0,0 through 999,0");
        assert_eq!(part1(&input), 1000);
        let input: Vec<Input> = test_input("turn on 499,499 through 500,500");
        assert_eq!(part1(&input), 4);

        let input: Vec<Input> = test_input("turn on 0,0 through 0,0");
        assert_eq!(part2(&input), 1);
        let input: Vec<Input> = test_input("toggle 0,0 through 999,999");
        assert_eq!(part2(&input), 2000000);
    }
}
