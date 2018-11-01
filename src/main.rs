#[allow(unused_assignments)]
extern crate rand;

use rand::prelude::*;

#[derive(Debug)]
struct Player {
    name: String,
    hp: i32,
    atk: i32,
    def: i32,
}

impl Player {
    fn is_alive(&self) -> bool {
        if self.hp > 0 {
            return true;
        } else {
            return false;
        }
    }
    fn new(name: String, hp: i32, atk: i32, def: i32) -> Player {
        Player { name, hp, atk, def }
    }
}

fn fight(player1: &mut Player, player2: &mut Player) {
    let mut done = false;
    let mut round = 1;
    let mut rng = thread_rng();
    let mut randatk: i32 = 0;

    while !done {
        println!("!! round {} begin !!", round);
        randatk = player2.atk - rng.gen_range(0, player1.def);
        player1.hp -= randatk;
        println!("{} attacked {} for {}", player2.name, player1.name, randatk);
        println!("{} has {} hp", player1.name, player1.hp);
        if player1.is_alive() == false {
            done = true;
            break;
        }
        randatk = player1.atk - rng.gen_range(0, player2.def);
        player2.hp -= randatk;
        println!("{} attacked {} for {}", player1.name, player2.name, randatk);
        println!("{} has {} hp", player2.name, player2.hp);
        if player2.is_alive() == false {
            done = true;
            break;
        }
        round += 1;
    }
    if player1.is_alive() {
        println!("{} wins!", player1.name)
    } else {
        println!("{} wins!", player2.name)
    }
}

fn main() {
    let mut player1 = Player::new(String::from("tim"), 100, 10, 10);
    let mut player2 = Player::new(String::from("jim"), 100, 10, 10);

    fight(&mut player1, &mut player2);

    
}
