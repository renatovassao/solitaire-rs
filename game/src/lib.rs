use deck::{Suit, Rank, Card, Deck};

#[derive(Debug)]
struct Tableau {
    stack: Vec<Card>
}

impl Tableau {
    pub fn new(stack: Vec<Card>) -> Tableau {
        Tableau {
            stack
        }
    }

    pub fn push(&mut self, mut card: Card) -> bool {
        if self.stack.len() == 0 {
            if card.get_rank() == Rank::King {
                self.stack.push(card);
                return true;
            } else {
                return false;
            }
        } else {
            let last = self.stack.last().unwrap();

            if last.get_rank() as i8 + 1 != card.get_rank() as i8 {
                return false;
            }

            if (
                (last.get_suit() == Suit::Diamonds || last.get_suit() == Suit::Hearts) && (card.get_suit() == Suit::Clubs || card.get_suit() == Suit::Spades)
            ) || (
                (last.get_suit() == Suit::Clubs || last.get_suit() == Suit::Spades) && (card.get_suit() == Suit::Diamonds || card.get_suit() == Suit::Hearts)
            ) {
                self.stack.push(card);
                return true;
            } else {
                return false;
            }
        }
    }
}

#[derive(Debug)]
struct Foundation {
    stack: Vec<Card>,
    suit: Suit,
}

impl Foundation {
    pub fn new(suit: Suit) -> Foundation {
        Foundation {
            suit,
            stack: Vec::with_capacity(13),
        }
    }
}

#[derive(Debug)]
struct Waste {
    stack: Vec<Card>
}

impl Waste {
    pub fn new() -> Waste {
        Waste {
            stack: Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.stack.pop()
    }

    pub fn push(&mut self, mut card: Card) {
        card.open();
        self.stack.push(card);
    }
}


#[derive(Debug)]
struct Stock {
    stack: Vec<Card>
}

impl Stock {
    pub fn new(stack: Vec<Card>) -> Stock {
        Stock {
            stack
        }
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.stack.pop()
    }

    pub fn push(&mut self, mut card: Card) {
        card.close();
        self.stack.push(card);
    }
}

#[derive(Copy, Clone, Debug)]
pub enum DealSize {
    One = 1,
    Three = 3,
}


#[derive(Debug)]
pub struct Game {
    p_1: Tableau,
    p_2: Tableau,
    p_3: Tableau,
    p_4: Tableau,
    p_5: Tableau,
    p_6: Tableau,
    p_7: Tableau,

    clubs_foundation: Foundation,
    diamonds_foundation: Foundation,
    hearts_foundation: Foundation,
    spades_foundation: Foundation,

    waste: Waste,
    stock: Stock,

    deal_size: DealSize
}

impl Game {
    pub fn new(deal_size: DealSize) -> Game {
        let mut deck = Deck::new();

        Game {
            p_1: Tableau::new(vec![deck.deal(true)]),
            p_2: Tableau::new(vec![deck.deal(false), deck.deal(true)]),
            p_3: Tableau::new(vec![deck.deal(false), deck.deal(true), deck.deal(true)]),
            p_4: Tableau::new(vec![deck.deal(false), deck.deal(true), deck.deal(true), deck.deal(true)]),
            p_5: Tableau::new(vec![deck.deal(false), deck.deal(true), deck.deal(true), deck.deal(true), deck.deal(true)]),
            p_6: Tableau::new(vec![deck.deal(false), deck.deal(true), deck.deal(true), deck.deal(true), deck.deal(true), deck.deal(true)]),
            p_7: Tableau::new(vec![deck.deal(false), deck.deal(true), deck.deal(true), deck.deal(true), deck.deal(true), deck.deal(true), deck.deal(true)]),

            clubs_foundation: Foundation::new(Suit::Clubs),
            diamonds_foundation: Foundation::new(Suit::Diamonds),
            hearts_foundation: Foundation::new(Suit::Hearts),
            spades_foundation: Foundation::new(Suit::Spades),

            waste: Waste::new(),
            stock: Stock::new(deck.cards),

            deal_size
        }
    }

    fn go_back(&mut self) {
        for _ in 0..self.waste.len() {
            self.stock.push(self.waste.pop().unwrap());
        }
    }

    pub fn deal(&mut self) {
        if self.stock.len() > 0 {
            for _ in 0..self.deal_size as u8 {
                self.waste.push(self.stock.pop().unwrap());
            }
        } else {
            self.go_back();
        }
    }
}