extern crate rand;

use rand::{thread_rng, Rng};

use cards;

#[derive(Eq, PartialEq, Hash, Debug)]
enum Phase {
    Explore,
}

pub struct Game {
    draw_pile: Vec<cards::Card>,
    discard_pile: Vec<cards::Card>,
}

impl Game {
    pub fn new(draw_pile: Vec<cards::Card>) -> Game {
        let mut draw_pile = draw_pile;
        let mut rng = thread_rng();
        rng.shuffle(&mut draw_pile);
        Game {
            draw_pile: draw_pile,
            discard_pile: vec![],
        }
    }

    pub fn draw(&mut self) -> cards::Card {
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

    pub fn discard(&mut self, card: cards::Card) {
        self.discard_pile.push(card);
    }
}
