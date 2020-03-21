use crate::game::{Pile, MAX_TABLEAU_SIZE};
use crate::deck::{Card, Suit, Rank};

pub struct Foundation {
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
            if last.get_numbered_rank() + 1 != card.get_numbered_rank() {
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