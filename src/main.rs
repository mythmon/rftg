use std::default::Default;

fn main() {
    for card in get_cards() {
        println!("{:?}", card);
    }
}

#[derive(Debug)]
enum CardType {
    World,
}

#[derive(Debug)]
enum Good {
    Novelty,
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

fn get_cards() -> Vec<Card> {
    vec![
        Card {
            name: "Alien Robot Sentry".to_string(),
            military_cost: 2,
            victory_points: 2,
            good: Some(Good::AlienTechnology),
            production: Some(Production::Windfall),
            ..Default::default()
        },
        Card {
            name: "Aquatic Uplift Race".to_string(),
            military_cost: 2,
            victory_points: 2,
            ..Default::default()
        },
        Card {
            name: "Asteroid Belt".to_string(),
            trade_cost: 2,
            victory_points: 1,
            good: Some(Good::RareElements),
            production: Some(Production::Windfall),
            ..Default::default()
        },
        Card {
            name: "Avian Uplift Race".to_string(),
            military_cost: 2,
            victory_points: 2,
            good: Some(Good::Genes),
            production: Some(Production::Windfall),
            ..Default::default()
        },
        Card {
            name: "Deserted Alien Colony".to_string(),
            trade_cost: 5,
            victory_points: 4,
            good: Some(Good::AlienTechnology),
            production: Some(Production::Windfall),
            ..Default::default()
        },
        Card {
            name: "Deserted Alien Library".to_string(),
            trade_cost: 6,
            victory_points: 5,
            good: Some(Good::AlienTechnology),
            production: Some(Production::Windfall),
            ..Default::default()
        },
        Card {
            name: "Deserted Alien Outpost".to_string(),
            trade_cost: 4,
            victory_points: 3,
            good: Some(Good::AlienTechnology),
            production: Some(Production::Windfall),
            ..Default::default()
        },
        Card {
            name: "Destroyed World".to_string(),
            trade_cost: 1,
            good: Some(Good::RareElements),
            production: Some(Production::Windfall),
            ..Default::default()
        },
        Card {
            name: "The Last of the  Uplift Gnarssh".to_string(),
            military_cost: 1,
            good: Some(Good::Genes),
            production: Some(Production::Windfall),
            ..Default::default()
        },
        Card {
            name: "Pre-Sentient Race".to_string(),
            trade_cost: 2,
            victory_points: 1,
            good: Some(Good::Genes),
            production: Some(Production::Windfall),
            ..Default::default()
        },
        Card {
            name: "Radioactive World".to_string(),
            trade_cost: 2,
            victory_points: 1,
            good: Some(Good::RareElements),
            production: Some(Production::Windfall),
            ..Default::default()
        },
        Card {
            name: "Rebel Base".to_string(),
            military_cost: 6,
            victory_points: 6,
            ..Default::default()
        },
        Card {
            name: "Rebel Fuel Cache".to_string(),
            military_cost: 1,
            victory_points: 1,
            ..Default::default()
        },
        Card {
            name: "Rebel Homeworld".to_string(),
            military_cost: 7,
            victory_points: 7,
            ..Default::default()
        },
        Card {
            name: "Reptile Uplift Race".to_string(),
            military_cost: 2,
            victory_points: 2,
            good: Some(Good::Genes),
            production: Some(Production::Windfall),
            ..Default::default()
        },
    ]
}
