use std::cmp::{Ordering};
use std::collections::BinaryHeap;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Enemy {
    hp: i32,
    dmg: i32,
}

impl Enemy {
    fn from_input(input: &Vec<String>) -> Self {
        let mut hp = 0;
        let mut dmg = 0;
        for l in input {
            let f: Vec<_> = l.split(": ").collect();
            let v = f[1].parse::<i32>().unwrap();
            match f[0] {
                "Hit Points" => { hp = v; },
                "Damage" => { dmg = v; },
                _ => panic!(),
            }
        }
        Self { hp, dmg }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Player {
    hp: i32,
    mana: i32,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    player: Player,
    enemy: Enemy,
    mana_spent: i32,
    turn: usize,
    shield: usize,
    poison: usize,
    recharge: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.mana_spent.cmp(&self.mana_spent)
            .then_with(|| other.turn.cmp(&self.turn))
            .then_with(|| self.player.hp.cmp(&other.player.hp))
            .then_with(|| other.enemy.hp.cmp(&self.enemy.hp))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn victory(&self) -> bool {
        self.enemy.hp == 0
    }
    fn next_states(&self, hardmode: bool) -> Vec<Self> {
        let mut next = self.clone();
        next.turn += 1;
        if hardmode && self.turn & 1 == 0 {
            next.player.hp -= 1;
            if next.player.hp == 0 {
                return vec![];
            }
        }
        let mut player_armor = 0;
        if next.shield > 0 {
            next.shield -= 1;
            player_armor = 7;
        }
        if next.poison > 0 {
            next.poison -= 1;
            next.enemy.hp = (next.enemy.hp - 3).max(0);
            if next.enemy.hp == 0 {
                return vec![next];
            }
        }
        if next.recharge > 0 {
            next.recharge -= 1;
            next.player.mana += 101;
        }
        if self.turn & 1 == 1 {
            // enemy's turn
            let attack = (next.enemy.dmg - player_armor).max(1);
            next.player.hp = (next.player.hp - attack).max(0);
            if next.player.hp == 0 {
                return vec![];
            }
            return vec![next];
        }
        //player's turn
        let mut out = Vec::new();
        if next.player.mana >= 53 {
            // magic missle
            let mut n = next.clone();
            n.player.mana -= 53;
            n.mana_spent += 53;
            n.enemy.hp = (n.enemy.hp - 4).max(0);
            out.push(n);
        }
        if next.player.mana >= 73 {
            // drain
            let mut n = next.clone();
            n.player.mana -= 73;
            n.mana_spent += 73;
            n.enemy.hp = (n.enemy.hp - 2).max(0);
            n.player.hp += 2;
            out.push(n);
        }
        if next.shield == 0 && next.player.mana >= 113 {
            // shield
            let mut n = next.clone();
            n.shield = 6;
            n.player.mana -= 113;
            n.mana_spent += 113;
            out.push(n);
        }
        if next.poison == 0 && next.player.mana >= 173 {
            // poison
            let mut n = next.clone();
            n.poison = 6;
            n.player.mana -= 173;
            n.mana_spent += 173;
            out.push(n);
        }
        if next.recharge == 0 && next.player.mana >= 229 {
            // recharge
            let mut n = next.clone();
            n.recharge = 5;
            n.player.mana -= 229;
            n.mana_spent += 229;
            out.push(n);
        }
        out
    }
}

fn battle(player: Player, enemy: Enemy, hardmode: bool) -> State {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State {
        player, enemy, mana_spent: 0, turn: 0, shield: 0, poison: 0, recharge: 0
    });
    while let Some(state) = heap.pop() {
        if state.victory() {
            return state;
        }
        for s in state.next_states(hardmode) {
            heap.push(s);
        }
    }
    panic!();
}

fn part1(input: &Vec<String>) -> i32 {
    let enemy = Enemy::from_input(input);
    let result = battle(Player { hp: 50, mana: 500 }, enemy, false);
    result.mana_spent
}

fn part2(input: &Vec<String>) -> i32 {
    let enemy = Enemy::from_input(input);
    let result = battle(Player { hp: 50, mana: 500 }, enemy, true);
    result.mana_spent
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
    fn day22_test() {
        let p = Player { hp: 10, mana: 250 };
        let e = Enemy { hp: 13, dmg: 8 };
        let result = battle(p, e, false);
        assert_eq!(result.player.hp, 2);
        assert_eq!(result.player.mana, 24);
        assert_eq!(result.turn, 4);
        let p = Player { hp: 10, mana: 250 };
        let e = Enemy { hp: 14, dmg: 8 };
        let result = battle(p, e, false);
        assert_eq!(result.turn, 10);
        assert_eq!(result.player.hp, 1);
        assert_eq!(result.player.mana, 114);
    }
}
