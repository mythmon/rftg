mod data;
mod cost;

use std::default::Default;
use std::fmt;

use utils;

pub use self::data::get_cards;
pub use self::cost::Cost;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Good {
    Novelty,
    RareElements,
    Genes,
    AlienTechnology,
}

impl utils::Variants for Good {
    fn variants() -> Vec<Good> {
        vec![
            Good::Novelty,
            Good::RareElements,
            Good::Genes,
            Good::AlienTechnology,
        ]
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Production {
    Windfall,
    Produces,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CardType {
    World,
    Development,
}

impl Default for CardType {
    fn default() -> CardType {
        CardType::World
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Power {
    ExploreSeeBonus(i32),
    ExploreKeepBonus(i32),

    DevelopDiscount(i32),
    DevelopDraw(i32),

    SettleMilitaryBonus(i32),
    SettleTradeDiscount(i32),
    SettleDiscountIfGood(i32, Option<Good>),
    SettleMilitaryIfGood(i32, Option<Good>),
    SettleMilitaryIfAttribute(i32, Attribute),
    SettleMilitaryAsTradeWithDiscount(i32),
    SettleDiscardForMilitary(i32),
    SettleDiscardToNegateTradeIfGood(Option<Good>),
}

#[derive(Debug, PartialEq, Clone)]
enum PowerType {
    Explore,
    // Develop,
    // Settle,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Attribute {
    Alien,
    Imperium,
    Rebel,
    Starter,
    Uplift,
}

impl utils::Variants for Attribute {
    fn variants() -> Vec<Attribute> {
        vec![
            Attribute::Alien,
            Attribute::Imperium,
            Attribute::Rebel,
            Attribute::Starter,
            Attribute::Uplift,
        ]
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Points {
    Simple(i32),
    TableauConditions(i32, Vec<Condition>),
    Military,
    PerVPChip(i32)
}

#[derive(Debug, Clone, PartialEq)]
enum Condition {
    CardType(CardType),
    PowerType(PowerType),
    Named(String),
    Attribute(Attribute),
    MinCost(Cost),
    // MaxCost(Cost),
    Produces(Production, Good),
    Good(Good),
    // Production(Production),
    Not(Box<Condition>),
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Card {
    pub name: String,
    pub card_type: CardType,
    pub cost: Cost,
    pub victory_points: Vec<Points>,
    pub produces: Option<(Production, Good)>,
    pub powers: Vec<Power>,
    pub attributes: Vec<Attribute>,
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

    fn trade_cost(mut self, cost: i32) -> Card {
        self.cost = Cost::Trade(cost);
        self
    }

    fn military_cost(mut self, cost: i32) -> Card {
        self.cost = Cost::Military(cost);
        self
    }

    fn add_points(mut self, points: Points) -> Card {
        self.victory_points.push(points);
        self
    }

    fn produces(mut self, production: Production, good: Good) -> Card {
        self.produces = Some((production, good));
        self
    }

    fn add_power(mut self, power: Power) -> Card {
        self.powers.push(power);
        self
    }

    fn add_attribute(mut self, attribute: Attribute) -> Card {
        self.attributes.push(attribute);
        self
    }
}

impl fmt::Display for Card {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut parts: Vec<String> = vec![self.name.clone()];

        let (left, right) = match self.card_type {
            CardType::World => ("(", ")"),
            CardType::Development => ("<", ">"),
        };

        parts.push(match self.cost {
            Cost::Military(m) => format!("{}{:?} - {} military{}", left, self.card_type, m, right),
            Cost::Trade(t) => format!("{}{:?} - {} trade{}", left, self.card_type, t, right),
            Cost::Free => format!("{}{:?} - free{}", left, self.card_type, right),
        });

        let mut has_complex = false;
        for points in (&self.victory_points).iter() {
            match points {
                &Points::Simple(n) => { parts.push(format!("{{{} VPs}}", n)); },
                _ => { has_complex = true; },
            }
        }

        if has_complex {
            parts.push("{?? VPs}".to_string());
        }

        match self.produces {
            None => {},
            Some((ref prod, ref good)) => parts.push(format!("{:?}: {:?}", prod, good)),
        }

        for power in self.powers.iter() {
            parts.push(format!("{:?}", power));
        }

        fmt.write_str(parts.connect(" ").as_slice()).ok().expect("Could not format string.");
        Result::Ok(())
    }
}
