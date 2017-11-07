use super::token::TokenPosition;

#[derive(Clone)]
pub struct Snapshot {
    pub pos: TokenPosition,
    pub index: usize,
}

impl Snapshot {
    fn new(pos: TokenPosition, index: usize) -> Snapshot {
        Snapshot {
            pos,
            index,
        }
    }
}

pub struct Tokenizer {
    pub pos: TokenPosition,
    index: usize,
    items: Vec<char>,
    snapshots: Vec<Snapshot>,
}

impl Tokenizer {
    pub fn new(items: Vec<char>) -> Tokenizer {
        Tokenizer {
            index: 0,
            pos: TokenPosition::default(),
            items,
            snapshots: Vec::new(),
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn advance(&mut self) -> bool {
        if let Some(a) = self.items.get(self.index + 1) {
            match *a {
                '\n' => {
                    self.pos.line += 1;
                    self.pos.col = 0;
                }
                _ => self.pos.col += 1
            }
            self.index += 1;
            true
        } else {
            false
        }
    }

    pub fn read(&mut self) -> Option<&char> {
        if self.advance() {
            Some(&self.items[self.index - 1])
        } else {
            None
        }
    }

    pub fn peek_n(&self, n: usize) -> Option<&char> {
        self.items.get(self.index + n)
    }

    pub fn peek(&self) -> Option<&char> {
        self.peek_n(0)
    }


}
