use super::tokenizer::Tokenizer;
use super::token::{Token, TokenType};

macro_rules! token {
    ($tokenizer:expr, $token_type:ident, $accum:expr) => {{
        token!($tokenizer , TokenType::$token_type, $accum)
    }};
    ($tokenizer:expr, $token_type:expr, $accum:expr) => {{
        let tokenizer  = $tokenizer  as &$crate::lua::lexer::tokenizer::Tokenizer;
        let token_type = $token_type as $crate::lua::lexer::token::TokenType;
        Some(Token::new(token_type, tokenizer.last_position(), $accum))
    }};
}

pub struct IntLiteralMatcher;

impl Matcher for IntLiteralMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut accum = String::new();
        while let Some(c) = tokenizer.next() {
            if c.is_digit(10) {
                accum.push(c.clone());
            } else {
                break
            }
        }

        if accum.is_empty() {
            None
        } else {
            token!(tokenizer, IntLiteral, accum)
        }
    }
}

pub struct StringLiteralMatcher;

impl Matcher for StringLiteralMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        if tokenizer.peek() == Some(&'"') {
            tokenizer.advance();

            let mut accum = String::new();
            while let Some(c) = tokenizer.next() {
                if c == '"' {
                    return token!(tokenizer, StringLiteral, accum);
                } else {
                    accum.push(c);
                }
            }
        }

        None
    }
}

pub trait Matcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token>;
}

pub struct IdentifierMatcher;

impl Matcher for IdentifierMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut accum = String::new();
        while let Some(c) = tokenizer.next() {
            if c.is_alphabetic() {
                accum.push(c);
            } else {
                break
            }
        }

        if accum.is_empty() {
            None
        } else {
            token!(tokenizer, Identifier, accum)
        }
    }
}

pub struct WhitespaceMatcher;

impl Matcher for WhitespaceMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut found = false;
        while !tokenizer.end() && tokenizer.peek().unwrap().is_whitespace() {
            found = true;
            tokenizer.next();
        }
        if found {
            token!(tokenizer, Whitespace, String::new())
        } else {
            None
        }
    }
}

pub struct ConstantCharMatcher {
    token_type: TokenType,
    constants: &'static [char],
}

impl ConstantCharMatcher {
    pub fn new(token_type: TokenType, constants: &'static [char]) -> Self {
        ConstantCharMatcher {
            token_type,
            constants,
        }
    }
}

impl Matcher for ConstantCharMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let c = tokenizer.peek().unwrap().clone();
        for constant in self.constants {
            if c == *constant {
                tokenizer.advance();
                return token!(tokenizer, self.token_type, constant.to_string())
            }
        }
        None
    }
}

pub struct ConstantStringMatcher {
    token_type: TokenType,
    constants: &'static [&'static str],
}

impl ConstantStringMatcher {
    pub fn new(token_type: TokenType, constants: &'static [&'static str]) -> Self {
        ConstantStringMatcher {
            token_type,
            constants,
        }
    }
}

impl Matcher for ConstantStringMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        for constant in self.constants {
            let dat = tokenizer.clone().take(constant.len());
            if dat.size_hint().1.unwrap() != constant.len() {
                return None
            }
            if dat.collect::<String>() == *constant {
                tokenizer.advance_n(constant.len());
                return token!(tokenizer, self.token_type.clone(), constant.to_string())
            }
        }
        None
    }
}
