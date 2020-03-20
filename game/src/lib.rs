use std::fmt;
use deck::{Suit, Rank, Card, Deck};

const MAX_TABLEAU_SIZE: usize = 13;

trait Pile<T> {
    fn can_push(&self, t: &T) -> bool;
    fn push(&mut self, t: T) -> Option<T>;
    fn pop(&mut self) -> Option<T>;
    fn len(&self) -> usize;
    fn last(&self) -> Option<&T>;
    fn get(&self, i: usize) -> Option<&T>;
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
    fn can_push(&self, card: &Card) -> bool {
        if !card.is_open() {
            return false;
        }

        if let Some(last) = self.stack.last() {
            if last.get_rank() as i8 - 1 != card.get_rank() as i8 {
                return false;
            }

            if (last.get_suit() == Suit::Diamonds || last.get_suit() == Suit::Hearts) && (card.get_suit() != Suit::Clubs && card.get_suit() != Suit::Spades) {
                return false;
            }

            if (last.get_suit() == Suit::Clubs || last.get_suit() == Suit::Spades) && (card.get_suit() != Suit::Diamonds && card.get_suit() != Suit::Hearts) {
                return false;
            }

        } else if card.get_rank() != Rank::King {
            return false;
        };

        true
    }

    fn push(&mut self, card: Card) -> Option<Card> {
        if self.can_push(&card) {
            self.stack.push(card);
            None
        } else {
            Some(card)
        }
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

    fn last(&self) -> Option<&Card> {
        self.stack.last()
    }

    fn get(&self, i: usize) -> Option<&Card> {
        self.stack.get(i)
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
    fn can_push(&self, card: &Card) -> bool {
        if !card.is_open() {
            return false;
        }

        if card.get_suit() != self.suit {
            return false;
        }

        if let Some(last) = self.stack.last() {
            if last.get_rank() as i8 + 1 != card.get_rank() as i8 {
                return false;
            }
        } else if card.get_rank() != Rank::Ace {
            return false;
        };

        true
    }

    fn push(&mut self, card: Card) -> Option<Card> {
        if self.can_push(&card) {
            self.stack.push(card);
            None
        } else {
            Some(card)
        }
    }

    fn pop(&mut self) -> Option<Card> {
        self.stack.pop()
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn last(&self) -> Option<&Card> {
        self.stack.last()
    }

    fn get(&self, i: usize) -> Option<&Card> {
        self.stack.get(i)
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
    fn can_push(&self, card: &Card) -> bool {
        card.is_open()
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn pop(&mut self) -> Option<Card> {
        self.stack.pop()
    }

    fn push(&mut self, card: Card) -> Option<Card> {
        if self.can_push(&card) {
            self.stack.push(card);
            None
        } else {
            Some(card)
        }
    }

    fn last(&self) -> Option<&Card> {
        self.stack.last()
    }

    fn get(&self, i: usize) -> Option<&Card> {
        self.stack.get(i)
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
    fn can_push(&self, card: &Card) -> bool {
        !card.is_open()
    }

    fn push(&mut self, card: Card) -> Option<Card> {
        if self.can_push(&card) {
            self.stack.push(card);
            None
        } else {
            Some(card)
        }
    }

    fn pop(&mut self) -> Option<Card> {
        self.stack.pop()
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn last(&self) -> Option<&Card> {
        self.stack.last()
    }

    fn get(&self, i: usize) -> Option<&Card> {
        self.stack.get(i)
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

    // pop methods

    fn pop_from_tableau(&mut self, tableau_number: usize) -> Option<Card> {
        match tableau_number {
            1 => self.tableau_1.pop(),
            2 => self.tableau_2.pop(),
            3 => self.tableau_3.pop(),
            4 => self.tableau_4.pop(),
            5 => self.tableau_5.pop(),
            6 => self.tableau_6.pop(),
            7 => self.tableau_7.pop(),
            _ => None
        }
    }

    fn pop_from_foundation(&mut self, foundation_suit: Suit) -> Option<Card> {
        match foundation_suit {
            Suit::Clubs => self.clubs_foundation.pop(),
            Suit::Diamonds => self.diamonds_foundation.pop(),
            Suit::Hearts => self.hearts_foundation.pop(),
            Suit::Spades => self.spades_foundation.pop(),
        }
    }

    fn pop_from_waste(&mut self) -> Option<Card> {
        self.waste.pop()
    }

    fn pop_from_stock(&mut self) -> Option<Card> {
        if let Some(mut card) = self.stock.pop() {
            card.open();
            return Some(card);
        }

        None
    }

    // get methods

    fn get_len_from_tableau(&self, tableau_number: usize) -> Option<usize> {
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

    fn get_from_tableau(&self, tableau_number: usize, i: usize) -> Option<&Card> {
        match tableau_number {
            1 => self.tableau_1.get(i),
            2 => self.tableau_2.get(i),
            3 => self.tableau_3.get(i),
            4 => self.tableau_4.get(i),
            5 => self.tableau_5.get(i),
            6 => self.tableau_6.get(i),
            7 => self.tableau_7.get(i),
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

    fn move_to_tableau(&mut self, tableau_number: usize, card: Card) -> Option<Card> {
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
        while let Some(mut card) = self.pop_from_waste() {
            card.close();
            self.move_to_stock(card);
        }
    }

    fn can_push_to_tableau(&self, tableau_number: usize, card: &Card) -> bool {
        match tableau_number {
            1 => self.tableau_1.can_push(card),
            2 => self.tableau_2.can_push(card),
            3 => self.tableau_3.can_push(card),
            4 => self.tableau_4.can_push(card),
            5 => self.tableau_5.can_push(card),
            6 => self.tableau_6.can_push(card),
            7 => self.tableau_7.can_push(card),
            _ => false
        }
    }

     // public methods

    pub fn deal(&mut self) {

        if self.stock.len() == 0 {
            self.go_back();
            return;
        }

        for _ in 0..self.get_deal_size() {
            if let Some(card) = self.pop_from_stock() {
                self.move_to_waste(card);
            }
        }
    }

    pub fn waste_to_foundation(&mut self, foundation_suit: Suit) -> bool {
        if let Some(waste_card) = self.pop_from_waste() {
            if let Some(card) = self.move_to_foundation(foundation_suit, waste_card) {
                self.move_to_waste(card);
                return false;
            }

            return true;
        }

        false
    }

    pub fn waste_to_tableau(&mut self, tableau_number: usize) -> bool {
        if let Some(waste_card) = self.pop_from_waste() {
            if let Some(card) = self.move_to_tableau(tableau_number, waste_card) {
                self.move_to_waste(card);
                return false;
            }

            return true;
        }

        false
    }

    pub fn tableau_to_foundation(&mut self, tableau_number: usize, foundation_suit: Suit) -> bool {
        if let Some(tableau_card) = self.pop_from_tableau(tableau_number) {
            if let Some(card) = self.move_to_foundation(foundation_suit, tableau_card) {
                self.move_to_tableau(tableau_number, card);
                return false;
            }
            return true;
        }

        false
    }

    pub fn tableau_to_tableau(&mut self, from_tableau_number: usize, to_tableau_number: usize, size: usize) -> bool {
        let i: isize;
        if let Some(tableau_size) = self.get_len_from_tableau(from_tableau_number) {
            if tableau_size < size {
                return false;
            }

            i = (tableau_size as isize) - (size as isize);
        } else {
            return false;
        }

        if let Some(first_card) = self.get_from_tableau(from_tableau_number, i as usize) {
            if !self.can_push_to_tableau(to_tableau_number, &first_card) {
                return false;
            }

            let mut tmp_stack: Vec<Card> = Vec::with_capacity(size);

            for _ in 0..size {
                tmp_stack.push(self.pop_from_tableau(from_tableau_number).unwrap())
            }

            while let Some(card) = tmp_stack.pop() {
                if let Some(card_2) = self.move_to_tableau(to_tableau_number, card) {
                    self.move_to_tableau(from_tableau_number, card_2).unwrap();
                }
            }

            return true;
        }

        false
    }

    pub fn foundation_to_tableau(&mut self, foundation_suit: Suit, tableau_number: usize) -> bool {
        if let Some(foundation_card) = self.pop_from_foundation(foundation_suit) {
            if let Some(card) = self.move_to_tableau(tableau_number, foundation_card) {
                self.move_to_foundation(foundation_suit, card);
                return false;
            }
            return true;
        }

        false
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // clubs
        write!(f, " ").unwrap();
        if let Some(last_club) = self.clubs_foundation.last() {
            last_club.fmt(f).unwrap();
        } else {
            write!(f, "{}", '\u{2663}').unwrap();
        }
        write!(f, " ").unwrap();

        // diamonds
        write!(f, " ").unwrap();
        if let Some(last_diamond) = self.diamonds_foundation.last() {
            last_diamond.fmt(f).unwrap();
        } else {
            write!(f, "{}", '\u{2666}').unwrap();
        }
        write!(f, " ").unwrap();

        // hearts
        write!(f, " ").unwrap();
        if let Some(last_heart) = self.hearts_foundation.last() {
            last_heart.fmt(f).unwrap();
        } else {
            write!(f, "{}", '\u{2665}').unwrap();
        }
        write!(f, " ").unwrap();

        // spades
        write!(f, " ").unwrap();
        if let Some(last_spade) = self.spades_foundation.last() {
            last_spade.fmt(f).unwrap();
        } else {
            write!(f, "{}", '\u{2660}').unwrap();
        }
        write!(f, " ").unwrap();

        // waste
        for i in (0..3).rev() {
            write!(f, " ").unwrap();
            if self.waste.len() <= i {
                write!(f, " ").unwrap();
                continue;
            }
            if let Some(waste_card) = self.waste.get(self.waste.len() - i - 1) {
                waste_card.fmt(f).unwrap();
            } else {
                write!(f, " ").unwrap();
            }
        }

        write!(f, " ").unwrap();

        // stock
        if let Some(stock_card) = self.stock.last() {
            stock_card.fmt(f).unwrap();
        } else {
            write!(f, "{}", '\u{1F0EA}').unwrap();
        }

        write!(f, " \n\n").unwrap();

        // tableaus
        for i in 0..14 {
            if self.tableau_1.len() < i && self.tableau_2.len() < i && self.tableau_3.len() < i && self.tableau_4.len() < i && self.tableau_5.len() < i && self.tableau_6.len() < i && self.tableau_6.len() < i {
                break;
            }
            write!(f, " ").unwrap();
            if let Some(tableau_card) = self.tableau_1.get(i) {
                tableau_card.fmt(f).unwrap();
            } else {
                write!(f, " ").unwrap();
            }
            write!(f, " ").unwrap();

            write!(f, " ").unwrap();
            if let Some(tableau_card) = self.tableau_2.get(i) {
                tableau_card.fmt(f).unwrap();
            } else {
                write!(f, " ").unwrap();
            }
            write!(f, " ").unwrap();

            write!(f, " ").unwrap();
            if let Some(tableau_card) = self.tableau_3.get(i) {
                tableau_card.fmt(f).unwrap();
            } else {
                write!(f, " ").unwrap();
            }
            write!(f, " ").unwrap();

            write!(f, " ").unwrap();
            if let Some(tableau_card) = self.tableau_4.get(i) {
                tableau_card.fmt(f).unwrap();
            } else {
                write!(f, " ").unwrap();
            }
            write!(f, " ").unwrap();

            write!(f, " ").unwrap();
            if let Some(tableau_card) = self.tableau_5.get(i) {
                tableau_card.fmt(f).unwrap();
            } else {
                write!(f, " ").unwrap();
            }
            write!(f, " ").unwrap();

            write!(f, " ").unwrap();
            if let Some(tableau_card) = self.tableau_6.get(i) {
                tableau_card.fmt(f).unwrap();
            } else {
                write!(f, " ").unwrap();
            }
            write!(f, " ").unwrap();

            write!(f, " ").unwrap();
            if let Some(tableau_card) = self.tableau_7.get(i) {
                tableau_card.fmt(f).unwrap();
            } else {
                write!(f, " ").unwrap();
            }
            write!(f, " ").unwrap();
            write!(f, "\n").unwrap();
        }
        write!(f, "\n")
    }
}
