use crate::deck::Suit;
use crate::menu::console;

pub enum Command {
    Deal,
    Help,
    Move(MoveSpec),
}

impl Command {
    pub fn parse(input: &str) -> Result<Command, String> {
        match input {
            "d" | "deal" => Ok(Command::Deal),
            "h" | "help" => Ok(Command::Help),
            "m" | "move" => match MoveSpec::read_and_parse() {
                Ok(spec) => Ok(Command::Move(spec)),
                Err(message) => Err(message)
            },
            _ => Err(format!("{} is not a valid command.", input)),
        }
    }
}

enum Location {
    Waste,
    Foundation(Suit),
    Tableau(usize)
}

impl Location {
    fn parse(input: &str) -> Result<Location, String> {
        match input {
            "w" | "waste" => Ok(Location::Waste),
            "c" | "clubs" => Ok(Location::Foundation(Suit::Clubs)),
            "d" | "diamonds" => Ok(Location::Foundation(Suit::Diamonds)),
            "h" | "hearts" => Ok(Location::Foundation(Suit::Hearts)),
            "s" | "spades" => Ok(Location::Foundation(Suit::Spades)),
            "1" | "t1" => Ok(Location::Tableau(1)),
            "2" | "t2" => Ok(Location::Tableau(2)),
            "3" | "t3" => Ok(Location::Tableau(3)),
            "4" | "t4" => Ok(Location::Tableau(4)),
            "5" | "t5" => Ok(Location::Tableau(5)),
            "6" | "t6" => Ok(Location::Tableau(6)),
            "7" | "t7" => Ok(Location::Tableau(7)),
            _ => Err(format!("{} is not a valid location.", input)),
        }
    }
}

pub enum MoveSpec {
    WasteToFoundation(Suit),
    WasteToTableau(usize),
    TableauToFoundation(usize, Suit),
    TableauToTableau(usize, usize, usize),
    FoundationToTableau(Suit, usize),
}

impl MoveSpec {
    fn parse(from: Location, to: Location) -> Result<MoveSpec, String> {
        match from {
            Location::Waste => {
                match to {
                    Location::Waste => Err(String::from("Cannot move to waste.")),
                    Location::Foundation(suit) => Ok(MoveSpec::WasteToFoundation(suit)),
                    Location::Tableau(n) => Ok(MoveSpec::WasteToTableau(n)),
                }
            },
            Location::Foundation(suit) => {
                match to {
                    Location::Waste => Err(String::from("Cannot move to waste.")),
                    Location::Foundation(_) => Err(String::from("Cannot move from foundation to foundation.")),
                    Location::Tableau(n) => Ok(MoveSpec::FoundationToTableau(suit, n)),
                }
            },
            Location::Tableau(n) => {
                match to {
                    Location::Waste => Err(String::from("Cannot move to waste.")),
                    Location::Foundation(suit) => Ok(MoveSpec::TableauToFoundation(n, suit)),
                    Location::Tableau(m) => Ok(MoveSpec::TableauToTableau(n, m, 0)),
                }
            },
        }
    }

    fn read_and_parse() -> Result<MoveSpec, String> {
        let input = console::read_line("from: ");

        match Location::parse(&input) {
            Ok(from) => {
                let input = console::read_line("to: ");

                match Location::parse(&input) {
                    Ok(to) => {
                        match Self::parse(from, to) {
                            Ok(spec) => {
                                match spec {
                                    MoveSpec::TableauToTableau(n, m, _) => {
                                        let input = console::read_line("size: ");

                                        match input.parse() {
                                            Ok(l) => Ok(MoveSpec::TableauToTableau(n, m, l)),
                                            Err(_) => Err(String::from("Invalid number of cards.")),
                                        }
                                    }
                                    _ => Ok(spec)
                                }
                            },
                            Err(message) => Err(message),
                        }
                    },
                    Err(message) => Err(message),
                }
            },
            Err(message) => Err(message),
        }
    }
}