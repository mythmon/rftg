extern crate rand;

use std::cell::RefCell;
use std::collections::HashMap;

use cards;
use game;
use utils::{self, Variants};

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
            game::Phase::Settle => self.settle(),
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
                    if card_ref.trade_cost > 0 && card_ref.trade_cost > buying_power {
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
                self.pay_trade_cost(price_to_pay);
            },
        }
    }

    fn settle(&mut self) {
        let mut trade_discount = 0;
        let mut military_power = 0;

        let mut can_convert_military_to_trade = false;
        let mut conversion_trade_discount = 0;

        let mut good_discounts: HashMap<Option<cards::Good>, i8> = HashMap::new();
        let mut good_military: HashMap<Option<cards::Good>, i8> = HashMap::new();
        let mut attr_military: HashMap<cards::Attribute, i8> = HashMap::new();

        for good in cards::Good::variants() {
            good_discounts[Some(good.clone())] = 0;
            good_military[Some(good.clone())] = 0;
        }
        good_discounts[None::<cards::Good>] = 0;
        good_military[None::<cards::Good>] = 0;

        for attr in cards::Attribute::variants() {
            attr_military[attr] = 0;
        }

        for card in self.tableau.iter() {
            for power in card.powers.iter() {
                match *power {
                    cards::Power::SettleMilitaryBonus(n) => { military_power += n },
                    cards::Power::SettleTradeDiscount(n) => { trade_discount += n },
                    cards::Power::SettleDiscountIfGood(n, ref good) => { good_discounts[good.clone()] += n },
                    cards::Power::SettleMilitaryIfGood(n, ref good) => { good_military[good.clone()] += n },
                    cards::Power::SettleMilitaryIfAttribute(n, ref attr) => { attr_military[attr.clone()] += n },
                    cards::Power::SettleMilitaryAsTradeWithDiscount(n) => { can_convert_military_to_trade = true; conversion_trade_discount += n },
                    _ => {},
                }
            }
        }

        let trade_power = (self.hand.len() as i8) + trade_discount - 1;

        println!("You have a military power of {}", military_power);
        println!("You have an effective buying power of {} ({} cards + {} discount - 1 bought)",
                 trade_power, self.hand.len(), trade_discount);

        if good_discounts.values().any(|n| { *n > 0 }) ||
           good_military.values().any(|n| { *n > 0 }) ||
           attr_military.values().any(|n| { *n > 0 })
        {
            println!("Additionally, you have");

            for (option_good, discount) in good_discounts.iter() {
                if *discount > 0 {
                    match *option_good {
                        Some(ref good) => println!("    A {} discount on world that produce {:?}.", discount, good),
                        None => println!("    A {} discount on worlds which do not product any good.", discount),
                    }
                }
            }

            for (option_good, military) in good_military.iter() {
                if *military > 0 {
                    match *option_good {
                        Some(ref good) => println!("    +{} military on world that produce {:?}.", military, good),
                        None => println!("    +{} military on worlds which do not product any good.", military),
                    }
                }
            }

            for (attr, military) in attr_military.iter() {
                if *military > 0 {
                    println!("    +{} military on world with the {:?} attribute.", military, attr);
                }
            }
        }


        let choice: Option<cards::Card>;
        loop {
            let settle_choices: Vec<&cards::Card> =
                self.hand.as_slice().iter()
                    .filter(|c| { c.card_type == cards::CardType::World })
                    .collect();
            match utils::select_optional(&settle_choices) {
                None => {
                    choice = None;
                    break;
                },
                Some(card_ref) => {
                    if (card_ref.trade_cost > 0 && card_ref.trade_cost > trade_power) ||
                       (card_ref.military_cost > 0 && card_ref.military_cost > military_power)
                    {
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

                if card.military_cost == 0 {
                    let price_to_pay = card.trade_cost;
                    self.pay_trade_cost(price_to_pay);
                } else {
                    println!("Your military conquers the world.");
                }
            },
        }
    }

    fn pay_trade_cost(&mut self, price_to_pay: i8) {
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
    }
}
