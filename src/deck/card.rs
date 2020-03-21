use std::fmt;

const CARD_BACK: char = '\u{1F0A0}';

#[derive(PartialEq, Clone)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Clubs => write!(f, "Clubs"),
            Suit::Diamonds => write!(f, "Diamonds"),
            Suit::Hearts => write!(f, "Hearts"),
            Suit::Spades => write!(f, "Spades"),
        }
    }
}

pub struct Card {
    rank: Rank,
    suit: Suit,
    open: bool,
    unicode: char,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit, unicode: char) -> Card {
        Card {
            rank,
            suit,
            open: false,
            unicode
        }
    }

    pub fn open(&mut self) {
        self.open = true;
    }

    pub fn close(&mut self) {
        self.open = false;
    }

    pub fn get_numbered_rank(&self) -> usize {
        match self.rank {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
        }
    }

    pub fn get_rank(&self) -> Rank {
        self.rank.clone()
    }

    pub fn get_suit(&self) -> Suit {
        self.suit.clone()
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.open {
            write!(f, "{}", self.unicode)
        } else {
            write!(f, "{}", CARD_BACK)
        }
    }
}
