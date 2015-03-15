#![feature(old_io)]
#![feature(core)]

extern crate rand;

use std::default::Default;
use std::old_io;
use std::str;
use rand::{thread_rng, Rng};
use std::ops;
use std::fmt;

fn main() {
    explore();
}
fn show_hand() {
}

fn explore() {
    let mut rng = thread_rng();
    let mut draw_pile: Vec<Card> = get_cards();
    let mut discard_pile: Vec<Card> = vec![];
    let mut hand: Vec<Card> = vec![];
    let mut explore_cards: Vec<Card> = vec![];

    rng.shuffle(&mut draw_pile);

    println!("Your hand:");
    for card in hand.iter() {
        println!("\t{:?}", card);
    }
    println!("");

    explore_cards.push(draw_pile.pop().unwrap());
    explore_cards.push(draw_pile.pop().unwrap());

    println!("Exploring");
    for (i, card) in explore_cards.iter().enumerate() {
        println!("\t{}) {:?}", i + 1, card);
    }
    println!("");

    println!("Which do you want to keep?");
    let keep: usize = get_num(1..explore_cards.len());

    for (i, card) in explore_cards.drain().enumerate() {
        if i == keep - 1 {
            hand.push(card)
        } else {
            discard_pile.push(card)
        }
    }

    println!("Your hand:");
    for card in hand {
        println!("\t{:?}", card);
    }

    println!("The discard has {} cards.", discard_pile.len());
}

fn get_num<T, U>(valid: U) -> T
        where T: str::FromStr + fmt::Debug + PartialOrd,
            U: Contains<T> + fmt::Debug {
    loop {
        print!("n = ");
        let input = old_io::stdin().read_line().ok().expect("Failed to read line");

        let num: T = match input.trim().parse().ok() {
            Some(num) => num,
            None => {
                println!("That's not a number!");
                continue;
            }
        };

        if !valid.contains(&num) {
            println!("That number is not in the range {:?}!", valid);
            continue;
        }

        return num
    };
}

#[derive(Debug)]
enum CardType {
    World,
}

#[derive(Debug)]
enum Good {
    // Novelty,
    RareElements,
    Genes,
    AlienTechnology,
}

#[derive(Debug)]
enum Production {
    Windfall,
}

impl Default for CardType {
    fn default() -> CardType {
        CardType::World
    }
}

#[derive(Default, Debug)]
struct Card {
    name: String,
    card_type: CardType,
    trade_cost: i8,
    military_cost: i8,
    victory_points: i8,
    good: Option<Good>,
    production: Option<Production>,
}

impl Card {
    fn new(name: &str) -> Card {
        Card {
            name: name.to_string(),
            ..Default::default()
        }
    }

    fn trade_cost(mut self, cost: i8) -> Card {
        self.trade_cost = cost;
        self
    }

    fn military_cost(mut self, cost: i8) -> Card {
        self.military_cost = cost;
        self
    }

    fn victory_points(mut self, cost: i8) -> Card {
        self.victory_points = cost;
        self
    }

    fn good(mut self, good: Good) -> Card {
        self.good = Some(good);
        self
    }

    fn production(mut self, production: Production) -> Card {
        self.production = Some(production);
        self
    }
}

fn get_cards() -> Vec<Card> {
    vec![
        Card::new("Alien Robot Sentry")
            .military_cost(2)
            .victory_points(2)
            .good(Good::AlienTechnology)
            .production(Production::Windfall),

        Card::new("Aquatic Uplift Race")
            .military_cost(2)
            .victory_points(2),

        Card::new("Asteroid Belt")
            .trade_cost(2)
            .victory_points(1)
            .good(Good::RareElements)
            .production(Production::Windfall),

        Card::new("Avian Uplift Race")
            .military_cost(2)
            .victory_points(2)
            .good(Good::Genes)
            .production(Production::Windfall),

        Card::new("Deserted Alien Colony")
            .trade_cost(5)
            .victory_points(4)
            .good(Good::AlienTechnology)
            .production(Production::Windfall),

        Card::new("Deserted Alien Library")
            .trade_cost(6)
            .victory_points(5)
            .good(Good::AlienTechnology)
            .production(Production::Windfall),

        Card::new("Deserted Alien Outpost")
            .trade_cost(4)
            .victory_points(3)
            .good(Good::AlienTechnology)
            .production(Production::Windfall),

        Card::new("Destroyed World")
            .trade_cost(1)
            .good(Good::RareElements)
            .production(Production::Windfall),

        Card::new("The Last of the  Uplift Gnarssh")
            .military_cost(1)
            .good(Good::Genes)
            .production(Production::Windfall),

        Card::new("Pre-Sentient Race")
            .trade_cost(2)
            .victory_points(1)
            .good(Good::Genes)
            .production(Production::Windfall),

        Card::new("Radioactive World")
            .trade_cost(2)
            .victory_points(1)
            .good(Good::RareElements)
            .production(Production::Windfall),

        Card::new("Rebel Base")
            .military_cost(6)
            .victory_points(6),

        Card::new("Rebel Fuel Cache")
            .military_cost(1)
            .victory_points(1),

        Card::new("Rebel Homeworld")
            .military_cost(7)
            .victory_points(7),

        Card::new("Reptile Uplift Race")
            .military_cost(2)
            .victory_points(2)
            .good(Good::Genes)
            .production(Production::Windfall),

    ]
}


trait Contains<T: PartialOrd> {
    fn contains(&self, needle: &T) -> bool;
}

impl<T: PartialOrd> Contains<T> for ops::Range<T> {
    fn contains(&self, needle: &T) -> bool {
        *needle >= self.start && *needle < self.end
    }
}

impl<T: PartialOrd> Contains<T> for ops::RangeFrom<T> {
    fn contains(&self, needle: &T) -> bool {
        *needle >= self.start
    }
}

impl<T: PartialOrd> Contains<T> for ops::RangeTo<T> {
    fn contains(&self, needle: &T) -> bool {
        *needle < self.end
    }
}

impl<T: PartialOrd> Contains<T> for ops::RangeFull {
    fn contains(&self, _: &T) -> bool {
        true
    }
}
