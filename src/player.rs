extern crate rand;

use std::io;
use std::io::Write;
use std::cell::RefCell;
use std::iter::FromIterator;
use std::num::ToPrimitive;

use cards;
use game;
use utils;

pub struct Player<'a> {
    game: &'a RefCell<game::Game>,
    hand: Vec<cards::Card>,
    tableau: Vec<cards::Card>,
}

impl<'a> Player<'a> {
    pub fn new(game: &'a RefCell<game::Game>) -> Player {
        Player {
            game: game,
            hand: vec![],
            tableau: vec![],
        }
    }

    pub fn draw_up_to(&mut self, up_to: usize) {
        while self.hand.len() < up_to {
            let c = self.game.borrow_mut().draw();
            self.hand.push(c);
        }
    }

    pub fn print_hand(&self) {
        if self.hand.len() > 0 {
            println!("Your hand:");
            for card in self.hand.iter() {
                println!("    {}", card);
            }
        } else {
            println!("You hand is empty.");
        }
        println!("");
    }

    pub fn print_tableau(&self) {
        if self.tableau.len() > 0 {
            println!("Your tableau:");
            for card in self.tableau.iter() {
                println!("    {}", card);
            }
        } else {
            println!("You tableau is empty.");
        }
        println!("");
    }

    pub fn act(&mut self, phase: &game::Phase) {
        match *phase {
            game::Phase::Explore => self.explore(),
            game::Phase::Develop => self.develop(),
        }
    }

    fn explore(&mut self) {
        let mut explore_cards: Vec<cards::Card> = vec![];
        let mut game = self.game.borrow_mut();

        let mut num_to_see: i8 = 2;
        let mut num_to_keep: i8 = 1;

        for card in self.tableau.iter() {
            for power in card.powers.iter() {
                match *power {
                    cards::Power::ExploreSeeBonus(n) => num_to_see += n,
                    cards::Power::ExploreKeepBonus(n) => num_to_keep += n,
                }
            }
        }

        for _ in 0..num_to_see {
            explore_cards.push(game.draw());
        }

        println!("Exploring");
        for (i, card) in explore_cards.iter().enumerate() {
            println!("    {}) {}", i + 1, card);
        }
        println!("");

        let mut indexes_to_discard = Vec::from_iter(0..explore_cards.len());
        let mut indexes_to_keep = vec![];

        if num_to_keep > 0 {
            println!("Which do you want to keep?");
        }
        while num_to_keep.to_usize().unwrap() > indexes_to_keep.len() {
            let left = num_to_keep.to_usize().unwrap() - indexes_to_keep.len();
            write!(&mut io::stdout(), "({} left) ", left).ok().expect("Could not write to stdout!");
            io::stdout().flush().ok().expect("Could not flush stdout!");

            let choices: Vec<usize> = indexes_to_discard.iter().map(|n| { n + 1 }).collect();
            let to_keep = utils::get_num(&choices) - 1;
            indexes_to_discard.retain(|n| { *n != to_keep });
            indexes_to_keep.push(to_keep);
        }

        for (i, card) in explore_cards.drain().enumerate() {
            if indexes_to_discard.contains(&i) {
                game.discard(card);
            } else {
                assert!(indexes_to_keep.contains(&i));
                self.hand.push(card);
            }
        }
    }

    fn develop(&mut self) {
        println!("Developing");

        let choice: Option<cards::Card> = {
            let development_choices: Vec<&cards::Card> =
                self.hand.as_slice().iter()
                    .filter(|c| { c.card_type == cards::CardType::Development })
                    .collect();
            match utils::select_optional(&development_choices) {
                None => None,
                Some(card_ref) => Some((*card_ref).clone()),
            }
        };

        match choice {
            None => {},
            Some(card) => {
                self.tableau.push(card.clone());
                self.hand.retain(|c| { *c != card });
            },
        }
    }
}
