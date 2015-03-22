use std::default::Default;
use std::fmt;
use std::cmp;

use utils;

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
    Develop,
    Settle,
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
pub enum Cost {
    Free,
    Trade(i32),
    Military(i32),
}

impl Default for Cost {
    fn default() -> Cost {
        Cost::Free
    }
}

impl cmp::PartialOrd<Cost> for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self, other) {
            (&Cost::Free, &Cost::Free) => Some(cmp::Ordering::Equal),
            (&Cost::Free, _) => Some(cmp::Ordering::Less),
            (&Cost::Trade(n1), &Cost::Trade(n2)) => n1.partial_cmp(&n2),
            (&Cost::Trade(_), _) => None,
            (&Cost::Military(n1), &Cost::Military(n2)) => n1.partial_cmp(&n2),
            (&Cost::Military(_), _) => None,
        }
    }
}

impl cmp::PartialEq<i32> for Cost {
    fn eq(&self, scalar: &i32) -> bool {
        match *self {
            Cost::Free => 0.eq(scalar),
            Cost::Trade(n) => n.eq(scalar),
            Cost::Military(n) => n.eq(scalar),
        }
    }
}

impl cmp::PartialOrd<i32> for Cost {
    fn partial_cmp(&self, scalar: &i32) -> Option<cmp::Ordering> {
        match *self {
            Cost::Free => 0.partial_cmp(scalar),
            Cost::Trade(n) => n.partial_cmp(scalar),
            Cost::Military(n) => n.partial_cmp(scalar),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum CardPoints {
    Simple(i32),
    TableauConditions(i32, Vec<CardCondition>),
    Military,
    PerVPChip(i32)
}

#[derive(Debug, Clone, PartialEq)]
enum CardCondition {
    CardType(CardType),
    PowerType(PowerType),
    Named(String),
    Attribute(Attribute),
    MinCost(Cost),
    MaxCost(Cost),
    Produces(Production, Good),
    Good(Good),
    Production(Production),
    Not(Box<CardCondition>),
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Card {
    pub name: String,
    pub card_type: CardType,
    pub cost: Cost,
    pub victory_points: Vec<CardPoints>,
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

    fn add_points(mut self, points: CardPoints) -> Card {
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
                &CardPoints::Simple(n) => { parts.push(format!("{{{} VPs}}", n)); },
                _ => { has_complex = true; },
            }
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

pub fn get_cards() -> Vec<Card> {
    vec![
        Card::new("Alien Tech Institute")
            .card_type(CardType::Development)
            .trade_cost(6)
            .add_power(Power::SettleDiscountIfGood(-2, Some(Good::AlienTechnology)))
            .add_power(Power::SettleMilitaryIfGood(2, Some(Good::AlienTechnology)))
            .add_attribute(Attribute::Alien)
            .add_points(CardPoints::TableauConditions(3, vec![
                CardCondition::CardType(CardType::World),
                CardCondition::Produces(Production::Produces, Good::AlienTechnology),
            ]))
            .add_points(CardPoints::TableauConditions(2, vec![
                CardCondition::CardType(CardType::World),
                CardCondition::Produces(Production::Windfall, Good::AlienTechnology),
            ]))
            .add_points(CardPoints::TableauConditions(2, vec![
                CardCondition::Attribute(Attribute::Alien),
                CardCondition::Not(box CardCondition::Good(Good::AlienTechnology)),
            ])),

        Card::new("Alien Robot Scout Ship")
            .card_type(CardType::World)
            .military_cost(4)
            .add_points(CardPoints::Simple(2))
            .produces(Production::Windfall, Good::AlienTechnology)
            .add_power(Power::SettleMilitaryBonus(1))
            .add_attribute(Attribute::Alien),

        Card::new("Alien Robot Sentry")
            .card_type(CardType::World)
            .military_cost(2)
            .add_points(CardPoints::Simple(2))
            .produces(Production::Windfall, Good::AlienTechnology)
            .add_attribute(Attribute::Alien),

        Card::new("Alpha Centauri")
            .card_type(CardType::World)
            .trade_cost(2)
            .produces(Production::Windfall, Good::RareElements)
            .add_power(Power::SettleDiscountIfGood(1, Some(Good::RareElements)))
            .add_power(Power::SettleMilitaryIfGood(1, Some(Good::RareElements))),

        Card::new("Aquatic Uplift Race")
            .card_type(CardType::World)
            .military_cost(2)
            .add_points(CardPoints::Simple(2))
            .add_attribute(Attribute::Uplift),

        Card::new("Asteroid Belt")
            .card_type(CardType::World)
            .trade_cost(2)
            .add_points(CardPoints::Simple(1))
            .produces(Production::Windfall, Good::RareElements  ),

        Card::new("Avian Uplift Race")
            .card_type(CardType::World)
            .military_cost(2)
            .add_points(CardPoints::Simple(2))
            .produces(Production::Windfall, Good::Genes)
            .add_attribute(Attribute::Uplift),

        Card::new("Blaster Gem Mines")
            .card_type(CardType::World)
            .trade_cost(3)
            .add_points(CardPoints::Simple(2))
            .produces(Production::Windfall, Good::RareElements)
            .add_power(Power::SettleMilitaryBonus(1)),

        Card::new("Colony Ship")
            .card_type(CardType::Development)
            .trade_cost(2)
            .add_points(CardPoints::Simple(1))
            .add_power(Power::SettleDiscardToNegateTradeIfGood(None)),
            // Doubled

        Card::new("Contact Specialist")
            .card_type(CardType::Development)
            .trade_cost(1)
            .add_points(CardPoints::Simple(1))
            .add_power(Power::SettleMilitaryBonus(-1))
            .add_power(Power::SettleMilitaryAsTradeWithDiscount(1)),
            // Doubled

        Card::new("Deserted Alien Colony")
            .card_type(CardType::World)
            .trade_cost(5)
            .add_points(CardPoints::Simple(4))
            .produces(Production::Windfall, Good::AlienTechnology)
            .add_attribute(Attribute::Alien),

        Card::new("Deserted Alien Library")
            .card_type(CardType::World)
            .trade_cost(6)
            .add_points(CardPoints::Simple(5))
            .produces(Production::Windfall, Good::AlienTechnology)
            .add_attribute(Attribute::Alien),

        Card::new("Deserted Alien Outpost")
            .card_type(CardType::World)
            .trade_cost(4)
            .add_points(CardPoints::Simple(3))
            .produces(Production::Windfall, Good::AlienTechnology)
            .add_attribute(Attribute::Alien),

        Card::new("Destroyed World")
            .card_type(CardType::World)
            .trade_cost(1)
            .produces(Production::Windfall, Good::RareElements),

        Card::new("Drop Ships")
            .card_type(CardType::Development)
            .trade_cost(4)
            .add_points(CardPoints::Simple(2))
            .add_power(Power::SettleMilitaryBonus(3)),
            // Doubled

        Card::new("Empath World")
            .card_type(CardType::World)
            .trade_cost(1)
            .add_points(CardPoints::Simple(1))
            .produces(Production::Windfall, Good::Genes)
            .add_power(Power::SettleMilitaryBonus(-1)),

        Card::new("Expedition Force")
            .card_type(CardType::Development)
            .trade_cost(1)
            .add_points(CardPoints::Simple(1))
            .add_power(Power::ExploreSeeBonus(1))
            .add_power(Power::SettleMilitaryBonus(1)),

        Card::new("Former Penal Colony")
            .card_type(CardType::World)
            .military_cost(2)
            .add_points(CardPoints::Simple(1))
            .produces(Production::Windfall, Good::Novelty)
            .add_power(Power::SettleMilitaryBonus(1)),

        Card::new("Galactic Federation")
            .card_type(CardType::Development)
            .trade_cost(6)
            .add_power(Power::DevelopDiscount(2))
            .add_points(CardPoints::TableauConditions(1, vec![
                CardCondition::CardType(CardType::Development),
                CardCondition::MinCost(Cost::Trade(6)),
            ]))
            .add_points(CardPoints::TableauConditions(1, vec![
                CardCondition::CardType(CardType::Development),
            ])),

        Card::new("Galactic Imperium")
            .card_type(CardType::Development)
            .trade_cost(6)
            .add_power(Power::SettleMilitaryIfAttribute(4, Attribute::Rebel))
            .add_attribute(Attribute::Imperium)
            .add_points(CardPoints::TableauConditions(1, vec![
                CardCondition::Attribute(Attribute::Rebel),
                CardCondition::MinCost(Cost::Military(1)),
            ]))
            .add_points(CardPoints::TableauConditions(1, vec![
                CardCondition::MinCost(Cost::Military(1)),
            ])),

        Card::new("Galactic Renaissance")
            .card_type(CardType::Development)
            .trade_cost(6)
            .add_power(Power::ExploreSeeBonus(2))
            .add_power(Power::ExploreKeepBonus(1))
            .add_points(CardPoints::PerVPChip(3))
            .add_points(CardPoints::TableauConditions(1, vec![
                CardCondition::Named("Research Labs".to_string()),
            ]))
            .add_points(CardPoints::TableauConditions(1, vec![
                CardCondition::Named("Galactic Trendsetters".to_string()),
            ]))
            .add_points(CardPoints::TableauConditions(1, vec![
                CardCondition::Named("Artist Colony".to_string()),
            ])),

        Card::new("Galactic Survey: SETI")
            .card_type(CardType::Development)
            .trade_cost(6)
            .add_power(Power::ExploreSeeBonus(2))
            .add_points(CardPoints::TableauConditions(1, vec![
                CardCondition::PowerType(PowerType::Explore),
            ]))
            .add_points(CardPoints::TableauConditions(1, vec![
                CardCondition::CardType(CardType::World),
            ])),

        Card::new("Interstellar Bank")
            .card_type(CardType::Development)
            .trade_cost(2)
            .add_points(CardPoints::Simple(1))
            .add_power(Power::DevelopDraw(1)),
            // Doubled

        Card::new("Investment Credits")
            .card_type(CardType::Development)
            .trade_cost(1)
            .add_points(CardPoints::Simple(1))
            .add_power(Power::DevelopDiscount(1)),
            // Doubled

        Card::new("The Last of the  Uplift Gnarssh")
            .card_type(CardType::World)
            .military_cost(1)
            .produces(Production::Windfall, Good::Genes)
            .add_attribute(Attribute::Uplift),

        Card::new("Lost Alien Warship")
            .card_type(CardType::World)
            .military_cost(5)
            .add_points(CardPoints::Simple(3))
            .produces(Production::Windfall, Good::AlienTechnology)
            .add_power(Power::SettleMilitaryBonus(2))
            .add_attribute(Attribute::Alien),

        Card::new("New Galactic Order")
            .card_type(CardType::Development)
            .trade_cost(6)
            .add_power(Power::SettleMilitaryBonus(2))
            .add_points(CardPoints::Military),

        Card::new("New Military Tactics")
            .card_type(CardType::Development)
            .trade_cost(1)
            .add_points(CardPoints::Simple(1))
            .add_power(Power::SettleDiscardForMilitary(3)),
            // Doubled

        Card::new("New Sparta")
            .card_type(CardType::World)
            .military_cost(2)
            .add_points(CardPoints::Simple(1))
            .add_power(Power::SettleMilitaryBonus(2))
            .add_attribute(Attribute::Starter),

        Card::new("Pre-Sentient Race")
            .card_type(CardType::World)
            .trade_cost(2)
            .add_points(CardPoints::Simple(1))
            .produces(Production::Windfall, Good::Genes),

        Card::new("Radioactive World")
            .card_type(CardType::World)
            .trade_cost(2)
            .add_points(CardPoints::Simple(1))
            .produces(Production::Windfall, Good::RareElements),

        Card::new("Rebel Base")
            .card_type(CardType::World)
            .military_cost(6)
            .add_points(CardPoints::Simple(6))
            .add_attribute(Attribute::Rebel),

        Card::new("Rebel Fuel Cache")
            .card_type(CardType::World)
            .military_cost(1)
            .add_points(CardPoints::Simple(1))
            .add_attribute(Attribute::Rebel),

        Card::new("Rebel Homeworld")
            .card_type(CardType::World)
            .military_cost(7)
            .add_points(CardPoints::Simple(7))
            .add_attribute(Attribute::Rebel),

        Card::new("Rebel Outpost")
            .card_type(CardType::World)
            .military_cost(5)
            .add_points(CardPoints::Simple(5))
            .add_power(Power::SettleMilitaryBonus(1))
            .add_attribute(Attribute::Rebel),

        Card::new("Rebel Warrior Race")
            .card_type(CardType::World)
            .military_cost(3)
            .add_points(CardPoints::Simple(2))
            .add_power(Power::SettleMilitaryBonus(1))
            .add_attribute(Attribute::Rebel),

        Card::new("Refugee World")
            .card_type(CardType::World)
            .add_points(CardPoints::Simple(1))
            .produces(Production::Windfall, Good::Novelty)
            .add_power(Power::SettleMilitaryBonus(-1)),

        Card::new("Replicant Robots")
            .card_type(CardType::Development)
            .trade_cost(4)
            .add_points(CardPoints::Simple(2))
            .add_power(Power::SettleTradeDiscount(2)),

        Card::new("Reptile Uplift Race")
            .card_type(CardType::World)
            .military_cost(2)
            .add_points(CardPoints::Simple(2))
            .produces(Production::Windfall, Good::Genes)
            .add_attribute(Attribute::Uplift),

        Card::new("Space Marines")
            .card_type(CardType::Development)
            .trade_cost(2)
            .add_points(CardPoints::Simple(1))
            .add_power(Power::SettleMilitaryBonus(2)),
            // Doubled
    ]
}
