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
