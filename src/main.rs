#![feature(core)]
#![feature(collections)]

extern crate rand;

use std::default::Default;
use std::io;
use std::str;
use rand::{thread_rng, Rng};
use std::ops;
use std::fmt;
use std::cell::RefCell;

fn main() {
    let cards = get_cards();
    let game = Game::new(cards);
    let game_ref = &RefCell::new(game);
    let mut play_areas = vec![];
    let num_players = 2;


    for _ in 0..num_players {
        let mut pa = PlayArea::new(game_ref);
        pa.draw_up_to(2);
        play_areas.push(pa);
    }

    // println!("There are {} cards in the draw pile.", game_ref.borrow().draw_pile.len());

    println!("Player 1, Explore!");
    play_areas[0].explore();
}

struct Game {
    draw_pile: Vec<Card>,
    discard_pile: Vec<Card>,
}

impl Game {
    fn new(draw_pile: Vec<Card>) -> Game {
        Game {
            draw_pile: draw_pile,
            discard_pile: vec![],
        }
    }

    fn draw(&mut self) -> Card {
        if self.draw_pile.is_empty() {
            if self.discard_pile.is_empty() {
                panic!("Out of cards!");
            }
            let mut rng = thread_rng();
            self.draw_pile.append(&mut self.discard_pile);
            rng.shuffle(&mut self.draw_pile);
        }
        self.draw_pile.pop().unwrap()
    }

    fn discard(&mut self, card: Card) {
        self.discard_pile.push(card);
    }
}

struct PlayArea<'a> {
    game: &'a RefCell<Game>,
    hand: Vec<Card>,
}

impl<'a> PlayArea<'a> {
    fn new(game: &'a RefCell<Game>) -> PlayArea {
        PlayArea {
            game: game,
            hand: vec![],
        }
    }

    fn draw_up_to(&mut self, up_to: usize) {
        while self.hand.len() < up_to {
            let c = self.game.borrow_mut().draw();
            self.hand.push(c);
        }
    }

    fn explore(&mut self) {
        let mut explore_cards: Vec<Card> = vec![];
        let mut game = self.game.borrow_mut();

        println!("Your hand:");
        for card in self.hand.iter() {
            println!("\t{:?}", card);
        }
        println!("");

        explore_cards.push(game.draw());
        explore_cards.push(game.draw());

        println!("Exploring");
        for (i, card) in explore_cards.iter().enumerate() {
            println!("\t{}) {:?}", i + 1, card);
        }
        println!("");

        println!("Which do you want to keep?");
        let keep: usize = get_num(1..explore_cards.len());

        for (i, card) in explore_cards.drain().enumerate() {
            if i == keep - 1 {
                self.hand.push(card);
            } else {
                game.discard(card);
            }
        }

        println!("Your hand:");
        for card in self.hand.iter() {
            println!("\t{:?}", card);
        }
    }
}

fn get_num<T, U>(valid: U) -> T
        where T: str::FromStr + fmt::Debug + PartialOrd,
            U: Contains<T> + fmt::Debug {

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Error reading stdin.");

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
