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
    let mut play_areas = vec![];
    let num_players = 2;

    for _ in 0..num_players {
        let mut pa = player::Player::new(game_ref);
        pa.draw_up_to(3);

        play_areas.push(pa);
    }

    let phases = game::Phase::variants();

    loop {
        play_areas[0].print_hand();
        play_areas[0].print_tableau();

        println!("What phase would you like to do?");
        let phase = utils::select(&phases);

        match phase {
            &game::Phase::Explore => {
                play_areas[0].explore();
            }
        }

        println!("");
    }
    //
    // println!("Player 1, Explore!");
    // play_areas[0].explore();
}
