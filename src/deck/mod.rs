mod card;

pub use card::{Card, Rank, Suit};

pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    // TODO: randomize deck
    pub fn new() -> Deck {
        Deck {
            cards: vec![
                Card::new(Rank::Ace, Suit::Clubs, '\u{1F0D1}'),
                Card::new(Rank::Two, Suit::Clubs, '\u{1F0D2}'),
                Card::new(Rank::Three, Suit::Clubs, '\u{1F0D3}'),
                Card::new(Rank::Four, Suit::Clubs, '\u{1F0D4}'),
                Card::new(Rank::Five, Suit::Clubs, '\u{1F0D5}'),
                Card::new(Rank::Six, Suit::Clubs, '\u{1F0D6}'),
                Card::new(Rank::Seven, Suit::Clubs, '\u{1F0D7}'),
                Card::new(Rank::Eight, Suit::Clubs, '\u{1F0D8}'),
                Card::new(Rank::Nine, Suit::Clubs, '\u{1F0D9}'),
                Card::new(Rank::Ten, Suit::Clubs, '\u{1F0DA}'),
                Card::new(Rank::Jack, Suit::Clubs, '\u{1F0DB}'),
                Card::new(Rank::Queen, Suit::Clubs, '\u{1F0DD}'),
                Card::new(Rank::King, Suit::Clubs, '\u{1F0DE}'),
                Card::new(Rank::Ace, Suit::Diamonds, '\u{1F0C1}'),
                Card::new(Rank::Two, Suit::Diamonds, '\u{1F0C2}'),
                Card::new(Rank::Three, Suit::Diamonds, '\u{1F0C3}'),
                Card::new(Rank::Four, Suit::Diamonds, '\u{1F0C4}'),
                Card::new(Rank::Five, Suit::Diamonds, '\u{1F0C5}'),
                Card::new(Rank::Six, Suit::Diamonds, '\u{1F0C6}'),
                Card::new(Rank::Seven, Suit::Diamonds, '\u{1F0C7}'),
                Card::new(Rank::Eight, Suit::Diamonds, '\u{1F0C8}'),
                Card::new(Rank::Nine, Suit::Diamonds, '\u{1F0C9}'),
                Card::new(Rank::Ten, Suit::Diamonds, '\u{1F0CA}'),
                Card::new(Rank::Jack, Suit::Diamonds, '\u{1F0CB}'),
                Card::new(Rank::Queen, Suit::Diamonds, '\u{1F0CD}'),
                Card::new(Rank::King, Suit::Diamonds, '\u{1F0CE}'),
                Card::new(Rank::Ace, Suit::Hearts, '\u{1F0B1}'),
                Card::new(Rank::Two, Suit::Hearts, '\u{1F0B2}'),
                Card::new(Rank::Three, Suit::Hearts, '\u{1F0B3}'),
                Card::new(Rank::Four, Suit::Hearts, '\u{1F0B4}'),
                Card::new(Rank::Five, Suit::Hearts, '\u{1F0B5}'),
                Card::new(Rank::Six, Suit::Hearts, '\u{1F0B6}'),
                Card::new(Rank::Seven, Suit::Hearts, '\u{1F0B7}'),
                Card::new(Rank::Eight, Suit::Hearts, '\u{1F0B8}'),
                Card::new(Rank::Nine, Suit::Hearts, '\u{1F0B9}'),
                Card::new(Rank::Ten, Suit::Hearts, '\u{1F0BA}'),
                Card::new(Rank::Jack, Suit::Hearts, '\u{1F0BB}'),
                Card::new(Rank::Queen, Suit::Hearts, '\u{1F0BD}'),
                Card::new(Rank::King, Suit::Hearts, '\u{1F0BE}'),
                Card::new(Rank::Ace, Suit::Spades, '\u{1F0A1}'),
                Card::new(Rank::Two, Suit::Spades, '\u{1F0A2}'),
                Card::new(Rank::Three, Suit::Spades, '\u{1F0A3}'),
                Card::new(Rank::Four, Suit::Spades, '\u{1F0A4}'),
                Card::new(Rank::Five, Suit::Spades, '\u{1F0A5}'),
                Card::new(Rank::Six, Suit::Spades, '\u{1F0A6}'),
                Card::new(Rank::Seven, Suit::Spades, '\u{1F0A7}'),
                Card::new(Rank::Eight, Suit::Spades, '\u{1F0A8}'),
                Card::new(Rank::Nine, Suit::Spades, '\u{1F0A9}'),
                Card::new(Rank::Ten, Suit::Spades, '\u{1F0AA}'),
                Card::new(Rank::Jack, Suit::Spades, '\u{1F0AB}'),
                Card::new(Rank::Queen, Suit::Spades, '\u{1F0AD}'),
                Card::new(Rank::King, Suit::Spades, '\u{1F0AE}'),
            ],
        }
    }

    // TODO: handle unwrap
    pub fn deal(&mut self, open: bool) -> Card {
        let mut card = self.cards.pop().unwrap();
        if open {
            card.open();
        }
        card
    }
}
