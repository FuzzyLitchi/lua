#[derive(Debug)]
pub enum TokenType {
    IntLiteral,
    StringLiteral,
    BoolLiteral,
    Symbol,
    Identifier,
    Whitespace,
    EOF,
}

#[derive(Copy, Clone, Debug)]
pub struct TokenPosition {
    pub line: usize,
    pub col: usize,
}

impl TokenPosition {
    pub fn new(line: usize, col: usize) -> TokenPosition {
        TokenPosition {
            line,
            col,
        }
    }
}

impl Default for TokenPosition {
    fn default() -> Self {
        TokenPosition {
            line: 0,
            col: 0,
        }
    }
}

#[derive(Debug)]
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
