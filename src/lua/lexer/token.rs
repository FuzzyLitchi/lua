#[derive(Copy, Clone, Debug)]
pub enum TokenType {
    IntLiteral,
    StringLiteral,
    BoolLiteral,
    Symbol,
    Identifier,
    Whitespace,
    EOF,
}

//A token position starts at 1,1
//The leftmost character in some text is in collum 1
//The first line is line 1
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
            line: 1,
            col: 1,
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
