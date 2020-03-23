use crate::game::Pile;
use crate::deck::{Card, Suit, Rank};

pub struct Tableau {
    stack: Vec<Card>
}

impl Tableau {
    pub fn new(stack: Vec<Card>) -> Tableau {
        Tableau {
            stack
        }
    }
}

impl Pile for Tableau {
    fn can_push(&self, card: &Card) -> bool {
        if !card.is_open() {
            return false;
        }

        if let Some(last) = self.stack.last() {
            if last.get_numbered_rank() - 1 != card.get_numbered_rank() {
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