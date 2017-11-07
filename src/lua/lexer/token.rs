pub enum TokenType {
    IntLiteral,
    StringLiteral,
    BoolLiteral,
    Symbol,
    Identifier,
}

#[derive(Clone)]
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
