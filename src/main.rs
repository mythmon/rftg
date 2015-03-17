#![feature(core)]
#![feature(collections)]
#![feature(io)]

extern crate rand;

use std::cell::RefCell;

mod cards;
mod utils;
mod game;
mod player;

use utils::Variants;

fn main() {
    let cards = cards::get_cards();
    let game = game::Game::new(cards);
    let game_ref = &RefCell::new(game);
    let mut players = vec![];
    let num_players = 2;

    for _ in 0..num_players {
        let mut pa = player::Player::new(game_ref);
        pa.draw_up_to(3);

        players.push(pa);
    }

    let phases = game::Phase::variants();

    loop {
        players[0].print_hand();
        players[0].print_tableau();

        println!("What phase would you like to do?");
        let phase = utils::select(&phases);

        players[0].act(phase);

        println!("");
    }
    //
    // println!("Player 1, Explore!");
    // players[0].explore();
}
