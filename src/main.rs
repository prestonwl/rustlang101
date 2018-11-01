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

//methods
impl Player {
    //check if players HP <= 0
    fn is_alive(&self) -> bool {
        if self.hp > 0 {
            return true;
        } else {
            return false;
        }
    }
    // damage = attacker atk value - a random number between 0 and defenders def value
    fn damage_received(&self, atk: i32) -> i32 {
        let mut randatk: i32 = 0;
        let mut rng = thread_rng();
        randatk = atk - rng.gen_range(0, self.def);
        return randatk;
    }
}

// related functions
impl Player {
    // instanciate a new player
    fn new(name: String, hp: i32, atk: i32, def: i32) -> Player {
        Player { name, hp, atk, def }
    }
    //main game fighting loop
    fn fight(player1: &mut Player, player2: &mut Player) {
        let mut done = false;
        let mut round = 1;

        while !done {
            let mut damage = player1.damage_received(player2.atk);
            player1.hp -= damage;
            println!("{} attacked {} for {}", player2.name, player1.name, damage);
            println!("!! round {} begin !!", round);
            println!("{} has {} hp", player1.name, player1.hp);
            if player1.is_alive() == false {
                done = true;
                break;
            }
            damage = player2.damage_received(player1.atk);
            player2.hp -= damage;
            println!("{} attacked {} for {}", player2.name, player1.name, damage);
            println!("{} has {} hp", player2.name, player2.hp);
            if player2.is_alive() == false {
                done = true;
                break;
            }
            round += 1;
        }
    }
    // after fight loop if player1 HP is > 0 he wins, else player2 wins
    fn who_wins(player1: Player, player2: Player) {
        if player1.is_alive() {
            println!("{} has {} hp, {} wins!",player2.name, player2.hp, player1.name)
        } else {
            println!("{} has {} hp, {} wins!", player1.name, player1.hp, player2.name)
        }
    }
}

fn main() {
    let mut player1 = Player::new(String::from("tim"), 100, 10, 10);
    let mut player2 = Player::new(String::from("jim"), 100, 10, 10);
    Player::fight(&mut player1, &mut player2);
    Player::who_wins(player1, player2);
}
