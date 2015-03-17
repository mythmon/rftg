use std::default::Default;
use std::fmt;

#[derive(Debug)]
pub enum Good {
    Novelty,
    RareElements,
    Genes,
    AlienTechnology,
}

#[derive(Debug)]
pub enum Production {
    Windfall,
}

#[derive(Debug)]
pub enum CardType {
    World,
    Development,
}

impl Default for CardType {
    fn default() -> CardType {
        CardType::World
    }
}

#[derive(Debug)]
pub enum Power {
    ExploreSeeBonus(i8),
    ExploreKeepBonus(i8),
}

#[derive(Default, Debug)]
pub struct Card {
    name: String,
    card_type: CardType,
    trade_cost: i8,
    military_cost: i8,
    victory_points: i8,
    good: Option<Good>,
    production: Option<Production>,
    pub powers: Vec<Power>,
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

impl fmt::Display for Card {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut parts: Vec<String> = vec![self.name.clone()];

        parts.push(match self.card_type {
            CardType::World => {
                if self.military_cost > 0 {
                    assert!(self.trade_cost == 0);
                    format!("({:?} - {} military)", self.card_type, self.military_cost)
                } else if self.trade_cost > 0 {
                    format!("({:?} - {} trade)", self.card_type, self.trade_cost)
                } else {
                    format!("({:?} - free)", self.card_type)
                }
            },
            CardType::Development => {
                if self.military_cost > 0 {
                    assert!(self.trade_cost == 0);
                    format!("<{:?} - {} military>", self.card_type, self.military_cost)
                } else if self.trade_cost > 0 {
                    format!("<{:?} - {} trade>", self.card_type, self.trade_cost)
                } else {
                    format!("<{:?} - free>", self.card_type)
                }
            },
        });

        if self.victory_points > 0 {
            parts.push(format!("{{{} VPs}}", self.victory_points));
        }

        match (&self.production, &self.good) {
            (&Some(ref prod), &Some(ref good)) => {
                parts.push(format!("{:?}: {:?}", prod, good));
            },
            (&None, &None) => {},
            _ => panic!("Inconsistent good/production values!"),
        }

        for power in self.powers.iter() {
            parts.push(format!("{:?}", power));
        }

        fmt.write_str(parts.connect(" ").as_slice()).ok().expect("Could not format string.");
        Result::Ok(())
    }
}

pub fn get_cards() -> Vec<Card> {
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

        Card::new("Galactic Renaissance")
            .card_type(CardType::Development)
            .trade_cost(6)
            .add_power(Power::ExploreSeeBonus(2))
            .add_power(Power::ExploreKeepBonus(1)),
            // Special scoring
            //   * +1VP for every VP in chips, divided by 3 rounded down
            //   * +3VP for cards: name="Research Labs"
            //   * +3VP for cards: name="Galactic Trendsetters"
            //   * +3VP for cards: name="Artist Colony"

        Card::new("Galactic Survey: SETI")
            .card_type(CardType::Development)
            .trade_cost(6)
            .add_power(Power::ExploreSeeBonus(2)),
            // Special scoring
            //   * +1VP for cards: has_phase=1
            //   * +1VP for cards: card_type=World

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
    ]
}
