#![feature(core)]
#![feature(collections)]
#![feature(io)]
#![feature(box_syntax)]

extern crate rand;

use std::default::Default;
use std::io;
use std::io::Write;
use std::str;
use std::ops;
use std::fmt;
use std::cell::RefCell;
use std::iter::FromIterator;
use std::num::ToPrimitive;

use rand::{thread_rng, Rng};

fn main() {
    let cards = get_cards();
    let game = Game::new(cards);
    let game_ref = &RefCell::new(game);
    let mut play_areas = vec![];
    let num_players = 2;

    for _ in 0..num_players {
        let mut pa = PlayArea::new(game_ref);
        pa.draw_up_to(2);

        for _ in 0..4 {
            pa.tableau.push(game_ref.borrow_mut().draw());
        }

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
        let mut draw_pile = draw_pile;
        let mut rng = thread_rng();
        rng.shuffle(&mut draw_pile);
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
    tableau: Vec<Card>,
}

impl<'a> PlayArea<'a> {
    fn new(game: &'a RefCell<Game>) -> PlayArea {
        PlayArea {
            game: game,
            hand: vec![],
            tableau: vec![],
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

        println!("Your tableau:");
        for card in self.tableau.iter() {
            println!("\t{:?}", card);
        }
        println!("");

        println!("Your hand:");
        for card in self.hand.iter() {
            println!("\t{:?}", card);
        }
        println!("");

        let mut num_to_see: i8 = 2;
        let mut num_to_keep: i8 = 1;

        for card in self.tableau.iter() {
            for power in card.powers.iter() {
                match *power {
                    Power::ExploreSeeBonus(n) => num_to_see += n,
                    Power::ExploreKeepBonus(n) => num_to_keep += n,
                }
            }
        }

        for _ in 0..num_to_see {
            explore_cards.push(game.draw());
        }

        println!("Exploring");
        for (i, card) in explore_cards.iter().enumerate() {
            println!("\t{}) {:?}", i + 1, card);
        }
        println!("");

        let mut indexes_to_discard = Vec::from_iter(0..explore_cards.len());
        let mut indexes_to_keep = vec![];

        if num_to_keep > 0 {
            println!("Which do you want to keep? (0 to stop)");
        }
        while num_to_keep.to_usize().unwrap() > indexes_to_keep.len() {
            let left = num_to_keep.to_usize().unwrap() - indexes_to_keep.len();
            write!(&mut io::stdout(), "({} left) ", left).ok().expect("Could not write to stdout!");
            io::stdout().flush().ok().expect("Could not flush stdout!");

            let mut to_keep = get_num(&indexes_to_discard);
            if to_keep == 0 {
                break;
            }
            to_keep -= 1;
            indexes_to_keep.push(to_keep);
            indexes_to_discard.retain(|n| { *n != to_keep });
        }

        for (i, card) in explore_cards.drain().enumerate() {
            if indexes_to_discard.contains(&i) {
                game.discard(card);
            } else {
                assert!(indexes_to_keep.contains(&i));
                self.hand.push(card);
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
          U: Contains<T> + fmt::Debug,
{
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

// #[derive(Eq, PartialEq, Hash, Debug)]
// enum Phase {
//     Explore,
// }

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

#[derive(Debug)]
enum CardType {
    World,
    Development,
}

impl Default for CardType {
    fn default() -> CardType {
        CardType::World
    }
}

#[derive(Debug)]
enum Power {
    ExploreSeeBonus(i8),
    ExploreKeepBonus(i8),
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
    powers: Vec<Power>,
}

impl Card {
    fn new(name: &str) -> Card {
        Card {
            name: name.to_string(),
            ..Default::default()
        }
    }

    fn card_type(mut self, card_type: CardType) -> Card {
        self.card_type = card_type;
        self
    }

    fn trade_cost(mut self, cost: i8) -> Card {
        if self.military_cost != 0 {
            panic!("Cannot assign trade cost to card with military cost");
        }
        self.trade_cost = cost;
        self
    }

    fn military_cost(mut self, cost: i8) -> Card {
        if self.trade_cost != 0 {
            panic!("Cannot assign military cost to card with trade cost");
        }
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

    fn add_power(mut self, power: Power) -> Card {
        self.powers.push(power);
        self
    }
}

fn get_cards() -> Vec<Card> {
    vec![
        Card::new("Alien Robot Sentry")
            .card_type(CardType::World)
            .military_cost(2)
            .victory_points(2)
            .good(Good::AlienTechnology)
            .production(Production::Windfall),

        Card::new("Aquatic Uplift Race")
            .card_type(CardType::World)
            .military_cost(2)
            .victory_points(2),

        Card::new("Asteroid Belt")
            .card_type(CardType::World)
            .trade_cost(2)
            .victory_points(1)
            .good(Good::RareElements)
            .production(Production::Windfall),

        Card::new("Avian Uplift Race")
            .card_type(CardType::World)
            .military_cost(2)
            .victory_points(2)
            .good(Good::Genes)
            .production(Production::Windfall),

        Card::new("Deserted Alien Colony")
            .card_type(CardType::World)
            .trade_cost(5)
            .victory_points(4)
            .good(Good::AlienTechnology)
            .production(Production::Windfall),

        Card::new("Deserted Alien Library")
            .card_type(CardType::World)
            .trade_cost(6)
            .victory_points(5)
            .good(Good::AlienTechnology)
            .production(Production::Windfall),

        Card::new("Deserted Alien Outpost")
            .card_type(CardType::World)
            .trade_cost(4)
            .victory_points(3)
            .good(Good::AlienTechnology)
            .production(Production::Windfall),

        Card::new("Destroyed World")
            .card_type(CardType::World)
            .trade_cost(1)
            .good(Good::RareElements)
            .production(Production::Windfall),

        Card::new("The Last of the  Uplift Gnarssh")
            .card_type(CardType::World)
            .military_cost(1)
            .good(Good::Genes)
            .production(Production::Windfall),

        Card::new("Pre-Sentient Race")
            .card_type(CardType::World)
            .trade_cost(2)
            .victory_points(1)
            .good(Good::Genes)
            .production(Production::Windfall),

        Card::new("Radioactive World")
            .card_type(CardType::World)
            .trade_cost(2)
            .victory_points(1)
            .good(Good::RareElements)
            .production(Production::Windfall),

        Card::new("Rebel Base")
            .card_type(CardType::World)
            .military_cost(6)
            .victory_points(6),

        Card::new("Rebel Fuel Cache")
            .card_type(CardType::World)
            .military_cost(1)
            .victory_points(1),

        Card::new("Rebel Homeworld")
            .card_type(CardType::World)
            .military_cost(7)
            .victory_points(7),

        Card::new("Reptile Uplift Race")
            .card_type(CardType::World)
            .military_cost(2)
            .victory_points(2)
            .good(Good::Genes)
            .production(Production::Windfall),

        Card::new("Galactic Renaissance")
            .card_type(CardType::Development)
            .trade_cost(6)
            .add_power(Power::ExploreSeeBonus(2))
            .add_power(Power::ExploreKeepBonus(1))
    ]
}

trait Contains<T> {
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

impl<T> Contains<T> for ops::RangeFull {
    fn contains(&self, _: &T) -> bool {
        true
    }
}

impl<'a, T: Eq> Contains<T> for &'a Vec<T> {
    fn contains(&self, needle: &T) -> bool {
        for hay in self.iter() {
            if hay == needle {
                return true;
            }
        }
        false
    }
}
