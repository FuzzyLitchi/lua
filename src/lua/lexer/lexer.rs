use std::rc::Rc;
use super::token::{Token, TokenType};
use super::tokenizer::Tokenizer;
use super::matcher::*;

pub fn lexer(data: Vec<char>) -> Lexer {
    let tokenizer = Tokenizer::new(data);
    let mut lexer = Lexer::new(tokenizer);

    let matcher_int_literal = IntLiteralMatcher;
    lexer.matchers_mut().push(Rc::new(matcher_int_literal));

    let matcher_identifier = IdentifierMatcher;
    lexer.matchers_mut().push(Rc::new(matcher_identifier));

    let matcher_whitespace = WhitespaceMatcher;
    lexer.matchers_mut().push(Rc::new(matcher_whitespace));

    lexer
}

pub struct Lexer {
    tokenizer: Tokenizer,
    matchers: Vec<Rc<Matcher>>,
}

impl Lexer {
    pub fn new(tokenizer: Tokenizer) -> Lexer {
        Lexer {
            tokenizer,
            matchers: Vec::new(),
        }
    }

    pub fn match_token(&mut self) -> Option<Token> {
        for matcher in &mut self.matchers {
            match self.tokenizer.try_match_token(matcher.as_ref()) {
                Some(t) => return Some(t),
                None => continue,
            }
        }
        None
    }

    pub fn matchers(&self) -> &Vec<Rc<Matcher>> {
        &self.matchers
    }

    pub fn matchers_mut(&mut self) -> &mut Vec<Rc<Matcher>> {
        &mut self.matchers
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let token = match self.match_token() {
            Some(n) => n,
            None    => return None,
        };
        match token.token_type {
            TokenType::EOF => None,
            TokenType::Whitespace => {
                match self.next() {
                    Some(t) => Some(t),
                    None => None,
                }
            }
            _ => Some(token),
        }
    }
}
