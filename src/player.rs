extern crate rand;

use std::cell::RefCell;
use std::collections::HashMap;

use cards;
use game;
use utils::{self, Variants};

struct Capabilities {
    explore_to_see: i32,
    explore_to_keep: i32,

    develop_trade_power: i32,
    develop_trade_discount: i32,
    develop_draw_before: i32,

    settle_trade_power: i32,
    settle_trade_discount: i32,
    settle_military_power: i32,
    settle_good_discounts: HashMap<Option<cards::Good>, i32>,
    settle_good_military: HashMap<Option<cards::Good>, i32>,
    settle_attr_military: HashMap<cards::Attribute, i32>,
    settle_can_convert_military_to_trade: bool,
    settle_conversion_discount: i32,
    settle_discard_military: Vec<(cards::Card, i32)>,
    settle_discard_to_negate_trade_if_good: Vec<(cards::Card, Option<cards::Good>)>,
}

impl Capabilities {
    fn new(hand_size: i32) -> Capabilities {
        let mut good_discounts = HashMap::new();
        let mut good_military = HashMap::new();
        let mut attr_military = HashMap::new();

        for good in cards::Good::variants() {
            good_discounts.insert(Some(good.clone()), 0);
            good_military.insert(Some(good.clone()), 0);
        }
        good_discounts.insert(None::<cards::Good>, 0);
        good_military.insert(None::<cards::Good>, 0);

        for attr in cards::Attribute::variants() {
            attr_military.insert(attr, 0);
        }

        Capabilities {
            explore_to_see: 2,
            explore_to_keep: 1,

            develop_trade_power: hand_size - 1,
            develop_trade_discount: 0,
            develop_draw_before: 0,

            settle_trade_power: hand_size - 1,
            settle_trade_discount: 0,
            settle_military_power: 0,
            settle_good_discounts: good_discounts,
            settle_good_military: good_military,
            settle_attr_military: attr_military,
            settle_can_convert_military_to_trade: false,
            settle_conversion_discount: 0,
            settle_discard_military: vec![],
            settle_discard_to_negate_trade_if_good: vec![],
        }
    }
}

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

    fn get_capabilities(&self) -> Capabilities {
        let mut caps = Capabilities::new(self.hand.len() as i32);

        for card in self.tableau.iter() {
            for power in card.powers.iter() {
                match *power {
                    cards::Power::ExploreSeeBonus(n) => { caps.explore_to_see += n; },
                    cards::Power::ExploreKeepBonus(n) => { caps.explore_to_keep += n; },

                    cards::Power::DevelopDiscount(n) => {
                        caps.develop_trade_power += n;
                        caps.develop_trade_discount += n;
                    },
                    cards::Power::DevelopDraw(n) => { caps.develop_draw_before += n; },

                    cards::Power::SettleTradeDiscount(n) => {
                        caps.settle_trade_power += n;
                        caps.settle_trade_discount += n;
                    },
                    cards::Power::SettleMilitaryBonus(n) => { caps.settle_military_power += n; },
                    cards::Power::SettleDiscountIfGood(n, ref good) => {
                        caps.settle_good_discounts[good.clone()] += n;
                    },
                    cards::Power::SettleMilitaryIfGood(n, ref good) => {
                        caps.settle_good_military[good.clone()] += n;
                    },
                    cards::Power::SettleMilitaryIfAttribute(n, ref attr) => {
                        caps.settle_attr_military[attr.clone()] += n;
                    },
                    cards::Power::SettleMilitaryAsTradeWithDiscount(n) => {
                        caps.settle_can_convert_military_to_trade = true;
                        caps.settle_conversion_discount += n;
                    },
                    cards::Power::SettleDiscardForMilitary(n) => {
                        caps.settle_discard_military.push((card.clone(), n));
                    },
                    cards::Power::SettleDiscardToNegateTradeIfGood(ref good) => {
                        let g = {
                            match *good {
                                Some(ref good) => Some(good.clone()),
                                None => None,
                            }
                        };
                        caps.settle_discard_to_negate_trade_if_good.push((card.clone(), g));
                    },
                }
            }
        }

        caps
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
        let caps = self.get_capabilities();


        for _ in 0..(caps.explore_to_see) {
            explore_cards.push(game.draw());
        }

        println!("Choose cards to keep.");
        let keep_cards = { utils::select_many(&explore_cards, caps.explore_to_keep as usize) };
        for card in keep_cards {
            self.hand.push(card);
        }
        explore_cards.retain(|c| { !self.hand.contains(c) });
        for card in explore_cards {
            game.discard(card);
        }
    }

    fn develop(&mut self) {
        let caps = self.get_capabilities();

        if caps.develop_draw_before > 0 {
            println!("Drawing {} cards.", caps.develop_draw_before);
            let mut game_ref = self.game.borrow_mut();
            for _ in 0..(caps.develop_draw_before) {
                let card = game_ref.draw();
                println!("    {}", card);
                self.hand.push(card);
            }
        }

        println!("You have an effective trade power of {}", caps.develop_trade_power);
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
                    if card_ref.cost > caps.develop_trade_power {
                        println!("You can't afford that card.");
                    } else {
                        choice = Some((*card_ref).clone());
                        break;
                    }
                },
            }
        };

        match choice {
            None => {},
            Some(card) => {
                self.tableau.push(card.clone());
                self.hand.retain(|c| { *c != card });

                match card.cost {
                    cards::Cost::Trade(n) => self.pay_trade_cost(n - caps.settle_trade_discount),
                    cards::Cost::Free => {},
                    cards::Cost::Military(_) => panic!("Military cost where not expected."),
                }
            },
        }
    }

    fn settle(&mut self) {
        let caps = self.get_capabilities();

        println!("You have a military power of {}", caps.settle_military_power);
        println!("You have an effective buying power of {}", caps.settle_trade_power);

        if caps.settle_good_discounts.values().any(|n| { *n > 0 }) ||
           caps.settle_good_military.values().any(|n| { *n > 0 }) ||
           caps.settle_attr_military.values().any(|n| { *n > 0 })
        {
            println!("Additionally, you have");

            for (option_good, discount) in caps.settle_good_discounts.iter() {
                if *discount > 0 {
                    match *option_good {
                        Some(ref good) => println!("    A {} discount on world that produce {:?}.", discount, good),
                        None => println!("    A {} discount on worlds which do not product any good.", discount),
                    }
                }
            }

            for (option_good, military) in caps.settle_good_military.iter() {
                if *military > 0 {
                    match *option_good {
                        Some(ref good) => println!("    +{} military on world that produce {:?}.", military, good),
                        None => println!("    +{} military on worlds which do not product any good.", military),
                    }
                }
            }

            for (attr, military) in caps.settle_attr_military.iter() {
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
                    if card_ref.cost > caps.settle_trade_power || card_ref.cost > caps.settle_military_power {
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

                match card.cost {
                    cards::Cost::Trade(c) => self.pay_trade_cost(c),
                    cards::Cost::Military(_) => println!("Your military conquers the world."),
                    cards::Cost::Free => println!("You settle the world for free."),
                }
            },
        }
    }

    fn pay_trade_cost(&mut self, price_to_pay: i32) {
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
