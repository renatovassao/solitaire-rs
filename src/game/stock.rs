use crate::game::Pile;
use crate::deck::Card;

pub struct Stock {
    stack: Vec<Card>
}

impl Stock {
    pub fn new(stack: Vec<Card>) -> Stock {
        Stock {
            stack
        }
    }
}

impl Pile for Stock {
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