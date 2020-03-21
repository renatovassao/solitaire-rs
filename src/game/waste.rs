use crate::game::Pile;
use crate::deck::Card;

pub struct Waste {
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