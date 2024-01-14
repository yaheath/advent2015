use std::cmp::Reverse;
use std::ops::Add;
use std::vec::Vec;
use itertools::{Itertools, iproduct};
use lazy_static::lazy_static;
use ya_advent_lib::read::read_input;

#[derive(Clone)]
struct Item {
    cost: u64,
    dmg: i32,
    arm: i32,
}

impl Add for Item {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            cost: self.cost + other.cost,
            dmg: self.dmg + other.dmg,
            arm: self.arm + other.arm,
        }
    }
}

impl<'a,'b> Add<&'b Item> for &'a Item {
    type Output = Item;
    fn add(self, other: &'b Item) -> Item {
        Item {
            cost: self.cost + other.cost,
            dmg: self.dmg + other.dmg,
            arm: self.arm + other.arm,
        }
    }
}

impl<'b> Add<&'b Item> for Item {
    type Output = Item;
    fn add(self, other: &'b Item) -> Item {
        Item {
            cost: self.cost + other.cost,
            dmg: self.dmg + other.dmg,
            arm: self.arm + other.arm,
        }
    }
}

lazy_static! {
    static ref WEAPONS: Vec<Item> = vec![
        Item {cost: 8, dmg: 4, arm: 0},
        Item {cost: 10, dmg: 5, arm: 0},
        Item {cost: 25, dmg: 6, arm: 0},
        Item {cost: 40, dmg: 7, arm: 0},
        Item {cost: 74, dmg: 8, arm: 0},
    ];
}
lazy_static! {
    static ref ARMOR: Vec<Item> = vec![
        Item {cost: 0, dmg: 0, arm: 0},
        Item {cost: 13, dmg: 0, arm: 1},
        Item {cost: 31, dmg: 0, arm: 2},
        Item {cost: 53, dmg: 0, arm: 3},
        Item {cost: 75, dmg: 0, arm: 4},
        Item {cost: 102, dmg: 0, arm: 5},
    ];
}
lazy_static! {
    static ref RINGS: Vec<Item> = vec![
        Item {cost: 25, dmg: 1, arm: 0},
        Item {cost: 50, dmg: 2, arm: 0},
        Item {cost: 100, dmg: 3, arm: 0},
        Item {cost: 20, dmg: 0, arm: 1},
        Item {cost: 40, dmg: 0, arm: 2},
        Item {cost: 80, dmg: 0, arm: 3},
    ].into_iter()
        .powerset()
        .filter(|l| l.len() < 3)
        .map(|l| {
            if l.len() == 0 {
                Item {cost: 0, dmg: 0, arm: 0}
            }
            else if l.len() == 2 {
                &(l[0]) + &(l[1])
            }
            else {
                l[0].clone()
            }
        })
        .collect();
}

#[derive(Clone)]
struct Character {
    hp: i32,
    dmg: i32,
    arm: i32,
}

impl Character {
    fn from_input(input: &Vec<String>) -> Character {
        let mut hp = 0;
        let mut dmg = 0;
        let mut arm = 0;
        for l in input {
            let f: Vec<_> = l.split(": ").collect();
            let v = f[1].parse::<i32>().unwrap();
            match f[0] {
                "Hit Points" => { hp = v; },
                "Damage" => { dmg = v; },
                "Armor" => { arm = v; },
                _ => panic!(),
            }
        }
        Character { hp, dmg, arm }
    }
    fn player_equip(item: &Item) -> Character {
        Character { hp: 100, dmg: item.dmg, arm: item.arm }
    }
}

fn attack(attacker: &Character, defender: &mut Character) {
    let dmg = (attacker.dmg - defender.arm).max(1);
    defender.hp = (defender.hp - dmg).max(0);
}

fn battle(mut player: Character, mut enemy: Character) -> bool {
    loop {
        attack(&player, &mut enemy);
        if enemy.hp == 0 { return true; }
        attack(&enemy, &mut player);
        if player.hp == 0 { return false; }
    }
}

fn part1(input: &Vec<String>) -> u64 {
    let enemy = Character::from_input(input);
    let mut equips = iproduct!(WEAPONS.iter(), ARMOR.iter(), RINGS.iter())
        .map(|(a,b,c)| a + b + c)
        .collect::<Vec<_>>();
    equips.sort_unstable_by_key(|e| e.cost);
    equips.into_iter()
        .find(|e| {
            let player = Character::player_equip(e);
            battle(player, enemy.clone())
        })
        .unwrap()
        .cost
}

fn part2(input: &Vec<String>) -> u64 {
    let enemy = Character::from_input(input);
    let mut equips = iproduct!(WEAPONS.iter(), ARMOR.iter(), RINGS.iter())
        .map(|(a,b,c)| a + b + c)
        .collect::<Vec<_>>();
    equips.sort_unstable_by_key(|e| Reverse(e.cost));
    equips.into_iter()
        .find(|e| {
            let player = Character::player_equip(e);
            !battle(player, enemy.clone())
        })
        .unwrap()
        .cost
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day21_test() {
        let p = Character { hp: 8, dmg: 5, arm: 5 };
        let e = Character { hp: 12, dmg: 7, arm: 2 };
        assert_eq!(battle(p, e), true);
        //assert_eq!(part2(&input), 0);
    }
}
