pub enum TokenType {
    IntLiteral,
    StringLiteral,
    BoolLiteral,
    Symbol,
    Identifier,
    Whitespace,
}

#[derive(Copy, Clone)]
pub struct TokenPosition {
    pub line: usize,
    pub col: usize,
}

impl Default for TokenPosition {
    fn default() -> Self {
        TokenPosition {
            line: 0,
            col: 0,
        }
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub pos: TokenPosition,
    content: String,
}

impl Token {
    pub fn new(token_type: TokenType, pos: TokenPosition, content: String) -> Token {
        Token {
            token_type,
            pos,
            content,
        }
    }
}
