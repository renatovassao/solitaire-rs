use deck::{Suit, Rank, Card, Deck};

const MAX_TABLEAU_SIZE: usize = 13;

trait Pile<T> {
    fn push(&mut self, t: T) -> Option<T>;
    fn pop(&mut self) -> Option<T>;
    fn len(&self) -> usize;
}

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
}

impl Pile<Card> for Tableau {
    fn push(&mut self, card: Card) -> Option<Card> {
        if !card.is_open() {
            return Some(card);
        }

        if let Some(last) = self.stack.last() {
            if last.get_rank() as i8 - 1 != card.get_rank() as i8 {
                return Some(card);
            }

            if (last.get_suit() == Suit::Diamonds || last.get_suit() == Suit::Hearts) && (card.get_suit() != Suit::Clubs && card.get_suit() != Suit::Spades) {
                return Some(card);
            }

            if (last.get_suit() == Suit::Clubs || last.get_suit() == Suit::Spades) && (card.get_suit() != Suit::Diamonds && card.get_suit() != Suit::Hearts) {
                return Some(card);
            }

        } else if card.get_rank() != Rank::King {
            return Some(card);

        };

        self.stack.push(card);
        return None;
    }

    fn pop(&mut self) -> Option<Card> {
        if let Some(card) = self.stack.pop() {
            if let Some(mut card_2) = self.stack.pop() {
                card_2.open();
                self.stack.push(card_2);
            }
            return Some(card);
        }

        None
    }

    fn len(&self) -> usize {
        self.stack.len()
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
            stack: Vec::with_capacity(MAX_TABLEAU_SIZE),
        }
    }
}

impl Pile<Card> for Foundation {
    fn push(&mut self, card: Card) -> Option<Card> {
        if !card.is_open() {
            return Some(card);
        }

        if card.get_suit() != self.suit {
            return Some(card);
        }

        if let Some(last) = self.stack.last() {
            if last.get_rank() as i8 + 1 != card.get_rank() as i8 {
                return Some(card);
            }
        } else if card.get_rank() != Rank::Ace {
            return Some(card);
        };

        self.stack.push(card);
        return None;
    }

    fn pop(&mut self) -> Option<Card> {
        self.stack.pop()
    }

    fn len(&self) -> usize {
        self.stack.len()
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
}

impl Pile<Card> for Waste {
    fn len(&self) -> usize {
        self.stack.len()
    }

    fn pop(&mut self) -> Option<Card> {
        self.stack.pop()
    }

    fn push(&mut self, mut card: Card) -> Option<Card> {
        card.open();
        self.stack.push(card);
        return None;
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
}

impl Pile<Card> for Stock {
    fn len(&self) -> usize {
        self.stack.len()
    }

    fn pop(&mut self) -> Option<Card> {
        self.stack.pop()
    }

    fn push(&mut self, mut card: Card) -> Option<Card> {
        card.close();
        self.stack.push(card);
        return None;
    }
}

#[derive(Copy, Clone, Debug)]
pub enum DealSize {
    One = 1,
    Three = 3,
}


#[derive(Debug)]
pub struct Game {
    tableau_1: Tableau,
    tableau_2: Tableau,
    tableau_3: Tableau,
    tableau_4: Tableau,
    tableau_5: Tableau,
    tableau_6: Tableau,
    tableau_7: Tableau,

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
            tableau_1: Tableau::new(vec![deck.deal(true)]),
            tableau_2: Tableau::new(vec![deck.deal(false), deck.deal(true)]),
            tableau_3: Tableau::new(vec![deck.deal(false), deck.deal(false), deck.deal(true)]),
            tableau_4: Tableau::new(vec![deck.deal(false), deck.deal(false), deck.deal(false), deck.deal(true)]),
            tableau_5: Tableau::new(vec![deck.deal(false), deck.deal(false), deck.deal(false), deck.deal(false), deck.deal(true)]),
            tableau_6: Tableau::new(vec![deck.deal(false), deck.deal(false), deck.deal(false), deck.deal(false), deck.deal(false), deck.deal(true)]),
            tableau_7: Tableau::new(vec![deck.deal(false), deck.deal(false), deck.deal(false), deck.deal(false), deck.deal(false), deck.deal(false), deck.deal(true)]),

            clubs_foundation: Foundation::new(Suit::Clubs),
            diamonds_foundation: Foundation::new(Suit::Diamonds),
            hearts_foundation: Foundation::new(Suit::Hearts),
            spades_foundation: Foundation::new(Suit::Spades),

            waste: Waste::new(),
            stock: Stock::new(deck.cards),

            deal_size
        }
    }

    // get methods

    fn get_from_tableau(&mut self, tableau_number: u8) -> Option<Card> {
        match tableau_number {
            1 => self.tableau_1.stack.pop(),
            2 => self.tableau_2.stack.pop(),
            3 => self.tableau_3.stack.pop(),
            4 => self.tableau_4.stack.pop(),
            5 => self.tableau_5.stack.pop(),
            6 => self.tableau_6.stack.pop(),
            7 => self.tableau_7.stack.pop(),
            _ => None
        }
    }

    fn get_from_foundation(&mut self, foundation_suit: Suit) -> Option<Card> {
        match foundation_suit {
            Suit::Clubs => self.clubs_foundation.pop(),
            Suit::Diamonds => self.diamonds_foundation.pop(),
            Suit::Hearts => self.hearts_foundation.pop(),
            Suit::Spades => self.spades_foundation.pop(),
        }
    }

    fn get_from_waste(&mut self) -> Option<Card> {
        self.waste.pop()
    }

    fn get_from_stock(&mut self) -> Option<Card> {
        self.stock.pop()
    }

    fn get_len_from_tableau(&mut self, tableau_number: u8) -> Option<usize> {
        match tableau_number {
            1 => Some(self.tableau_1.len()),
            2 => Some(self.tableau_2.len()),
            3 => Some(self.tableau_3.len()),
            4 => Some(self.tableau_4.len()),
            5 => Some(self.tableau_5.len()),
            6 => Some(self.tableau_6.len()),
            7 => Some(self.tableau_7.len()),
            _ => None
        }
    }

    fn get_deal_size(&self) -> usize {
        self.deal_size as usize
    }

    // move methods

    fn move_to_foundation(&mut self, foundation_suit: Suit, card: Card) -> Option<Card> {
        match foundation_suit {
            Suit::Clubs => self.clubs_foundation.push(card),
            Suit::Diamonds => self.diamonds_foundation.push(card),
            Suit::Hearts => self.hearts_foundation.push(card),
            Suit::Spades => self.spades_foundation.push(card),
        }
    }

    fn move_to_tableau(&mut self, tableau_number: u8, card: Card) -> Option<Card> {
        match tableau_number {
            1 => self.tableau_1.push(card),
            2 => self.tableau_2.push(card),
            3 => self.tableau_3.push(card),
            4 => self.tableau_4.push(card),
            5 => self.tableau_5.push(card),
            6 => self.tableau_6.push(card),
            7 => self.tableau_7.push(card),
            _ => Some(card)
        }
    }

    fn move_to_waste(&mut self, card: Card) -> Option<Card> {
        self.waste.push(card)
    }

    fn move_to_stock(&mut self, card: Card) -> Option<Card> {
        self.stock.push(card)
    }

    // helper methods

    fn go_back(&mut self) {
        while let Some(card) = self.get_from_waste() {
            self.move_to_stock(card);
        }
    }

     // public methods

    pub fn deal(&mut self) {
        let n = self.get_deal_size();

        if n == 0 {
            self.go_back();
            return;
        }

        for _ in 0..n {
            if let Some(card) = self.get_from_stock() {
                self.move_to_waste(card);
            }
        }
    }

    pub fn waste_to_foundation(&mut self, foundation_suit: Suit) -> bool {
        if let Some(waste_card) = self.get_from_waste() {
            if let Some(card) = self.move_to_foundation(foundation_suit, waste_card) {
                self.move_to_waste(card);
                return false;
            }

            return true;
        }

        false
    }

    pub fn waste_to_tableau(&mut self, tableau_number: u8) -> bool {
        if let Some(waste_card) = self.get_from_waste() {
            if let Some(card) = self.move_to_tableau(tableau_number, waste_card) {
                self.move_to_waste(card);
                return false;
            }

            return true;
        }

        false
    }

    pub fn tableau_to_foundation(&mut self, tableau_number: u8, foundation_suit: Suit) -> bool {
        if let Some(tableau_card) = self.get_from_tableau(tableau_number) {
            if let Some(card) = self.move_to_foundation(foundation_suit, tableau_card) {
                self.move_to_tableau(tableau_number, card);
                return false;
            }
            return true;
        }

        false
    }

    pub fn tableau_to_tableau(&mut self, from_tableau_number: u8, to_tableau_number: u8, size: usize) -> bool {
        if size == 0 || 7 < size {
            return false;
        }

        if let Some(n) = self.get_len_from_tableau(from_tableau_number) {
            if n < size {
                return false;
            }

            let mut tmp_stack: Vec<Card> = Vec::with_capacity(size);

            for _ in 0..size {
                tmp_stack.push(self.get_from_tableau(from_tableau_number).unwrap())
            }

            let mut bad_move = false;

            while let Some(card) = tmp_stack.pop() {
                if bad_move {
                    self.move_to_tableau(from_tableau_number, card);
                    continue;
                }

                if let Some(card_2) = self.move_to_tableau(to_tableau_number, card) {
                    bad_move = true;
                    self.move_to_tableau(from_tableau_number, card_2);
                }
            }

            return true;
        }

        false
    }

    pub fn foundation_to_tableau(&mut self, foundation_suit: Suit, tableau_number: u8) -> bool {
        if let Some(foundation_card) = self.get_from_foundation(foundation_suit) {
            if let Some(card) = self.move_to_tableau(tableau_number, foundation_card) {
                self.move_to_foundation(foundation_suit, card);
                return false;
            }
            return true;
        }

        false
    }
}