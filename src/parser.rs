use crate::deck;

pub trait Parser<T> {
    fn parse(input: &str) -> Result<T, String>;
}

pub enum Command {
    Deal,
    Help,
    WasteToFoundation(deck::Suit),
    WasteToTableau(usize),
    TableauToFoundation(usize, deck::Suit),
    TableauToTableau(usize, usize, usize),
    FoundationToTableau(deck::Suit, usize),
}

impl Parser<Command> for Command {
    fn parse(input: &str) -> Result<Command, String> {
        match input {
            "d" | "deal" => Ok(Command::Deal),
            "h" | "help" => Ok(Command::Help),
            "wc" => Ok(Command::WasteToFoundation(deck::Suit::Clubs)),
            "wd" => Ok(Command::WasteToFoundation(deck::Suit::Diamonds)),
            "wh" => Ok(Command::WasteToFoundation(deck::Suit::Hearts)),
            "ws" => Ok(Command::WasteToFoundation(deck::Suit::Spades)),
            _ => Err(format!("{} is not a valid command.", input)),
        }
    }
}