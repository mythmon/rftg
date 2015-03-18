extern crate rand;

use std::cell::RefCell;

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

    pub fn act(&mut self, phase: game::Phase) {
        match phase {
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
                    _ => (),
                }
            }
        }

        for _ in 0..num_to_see {
            explore_cards.push(game.draw());
        }

        println!("Choose cards to keep.");
        let keep_cards = { utils::select_many(&explore_cards, num_to_keep as usize) };
        for card in keep_cards {
            self.hand.push(card);
        }
        explore_cards.retain(|c| { !self.hand.contains(c) });
        for card in explore_cards {
            game.discard(card);
        }
    }

    fn develop(&mut self) {
        let mut num_to_draw = 0;
        let mut discount = 0;

        for card in self.tableau.iter() {
            for power in card.powers.iter() {
                match *power {
                    cards::Power::DevelopDiscount(n) => discount += n,
                    cards::Power::DevelopDraw(n) => num_to_draw += n,
                    _ => (),
                }
            }
        }

        if num_to_draw > 0 {
            println!("Drawing {} cards.", num_to_draw);
            let mut game_ref = self.game.borrow_mut();
            for _ in 0..num_to_draw {
                let card = game_ref.draw();
                println!("    {}", card);
                self.hand.push(card);
            }
        }

        let buying_power = (self.hand.len() as i8) + discount - 1;

        println!("You have an effective buying power of {} ({} cards + {} discount - 1 bought)",
                buying_power, self.hand.len(), discount);
        println!("What would you like to develop?");
        println!("");

        let choice: Option<cards::Card>;
        loop {
            let development_choices: Vec<&cards::Card> =
                self.hand.as_slice().iter()
                    .filter(|c| { c.card_type == cards::CardType::Development })
                    .collect();
            match utils::select_optional(&development_choices) {
                None => {
                    choice = None;
                    break;
                },
                Some(card_ref) => {
                    if card_ref.trade_cost > buying_power {
                        println!("You can't afford that card.");
                    } else {
                        choice = Some((*card_ref).clone());
                        break;
                    }
                }
            }
        };

        match choice {
            None => {},
            Some(card) => {
                self.tableau.push(card.clone());
                self.hand.retain(|c| { *c != card });

                let price_to_pay: i8 = card.trade_cost - discount;
                if price_to_pay > 0 {
                    println!("Choose cards to use as payment.");
                    let payment_cards = { utils::select_many(&self.hand, price_to_pay as usize) };

                    let size_before = self.hand.len();
                    self.hand.retain(|c| { !payment_cards.contains(&c) });
                    assert!(size_before - (price_to_pay as usize) == self.hand.len());

                    for payment in payment_cards {
                        self.game.borrow_mut().discard(payment.clone());
                    }
                } else {
                    println!("Your cost is 0.");
                }
            },
        }
    }
}
