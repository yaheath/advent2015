use std::vec::Vec;
use ya_advent_lib::read::read_input;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::grid::Grid;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    On,
    Off,
    OnNext,
    OffNext,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Off,
            '#' => Cell::On,
            _ => panic!(),
        }
    }
}

fn step(grid: &mut Grid<Cell>, part2: bool) {
    for y in grid.y_bounds() {
        for x in grid.x_bounds() {
            let c = Coord2D::new(x, y);
            let onn = c.neighbors8().iter()
                .filter(|cc| grid.x_bounds().contains(&cc.x) && grid.y_bounds().contains(&cc.y))
                .filter(|cc| match grid.get(cc.x, cc.y) {
                    Cell::On | Cell::OffNext => true,
                    _ => false,
                })
                .count();
            let new = match grid.get(x, y) {
                Cell::On if onn >= 2 && onn <= 3 => Cell::On,
                Cell::On => Cell::OffNext,
                Cell::Off if onn == 3 => Cell::OnNext,
                Cell::Off => Cell::Off,
                _ => panic!(),
            };
            grid.set(x, y, new);
        }
    }
    grid.iter_mut().for_each(|v| match v {
        Cell::OnNext => { *v = Cell::On; },
        Cell::OffNext => { *v = Cell::Off; },
        _ => {},
    });
    if part2 {
        let w = grid.x_bounds().end;
        let h = grid.y_bounds().end;
        grid.set(0, 0, Cell::On);
        grid.set(0, h-1, Cell::On);
        grid.set(w-1, 0, Cell::On);
        grid.set(w-1, h-1, Cell::On);
    }
}

fn run(input: &[String], nsteps: usize, part2: bool) -> usize {
    let mut grid = mkgrid(input);
    if part2 {
        let w = grid.x_bounds().end;
        let h = grid.y_bounds().end;
        grid.set(0, 0, Cell::On);
        grid.set(0, h-1, Cell::On);
        grid.set(w-1, 0, Cell::On);
        grid.set(w-1, h-1, Cell::On);
    }
    for _ in 0..nsteps {
        step(&mut grid, part2);
    }
    grid.iter().filter(|&&c| c == Cell::On).count()
}

fn mkgrid(input: &[String]) -> Grid<Cell> {
    Grid::from_input(input, Cell::Off, 0)
}

fn part1(input: &[String]) -> usize {
    run(input, 100, false)
}

fn part2(input: &[String]) -> usize {
    run(input, 100, true)
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day18_test() {
        let input: Vec<String> = test_input(include_str!("day18.testinput"));
        assert_eq!(run(&input, 4, false), 4);
        assert_eq!(run(&input, 5, true), 17);
    }
}
