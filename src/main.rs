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
    RareElement,
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
            name: "Destroyed World".to_string(),
            trade_cost: 1,
            good: Some(Good::RareElement),
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
    ]
}
