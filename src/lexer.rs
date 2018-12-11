use std::iter::Peekable;

use crate::{Syllable, Word};

#[derive(Debug)]
pub enum Token {
    Any(String),
    Word(Word),
}

pub struct Lexer<I: Iterator<Item=char>> {
    it: Peekable<I>,
}

impl<I> Lexer<I> where I: Iterator<Item=char> {
    pub fn new(it: I) -> Lexer<I> {
        Lexer { it: it.peekable() }
    }

    fn take_while_is_hangeul(&mut self) -> Option<String> {
        let mut res = String::new();

        while let Some(&c) = self.it.peek() {
            if Syllable::is_valid(c) {
                res.push(c);
                self.it.next();
            } else {
                break;
            }
        }

        if !res.is_empty() {
            Some(res)
        } else {
            None
        }
    }

    fn take_while_is_not_hangeul(&mut self) -> Option<String> {
        let mut res = String::new();

        while let Some(&c) = self.it.peek() {
            if !Syllable::is_valid(c) {
                res.push(c);
                self.it.next();
            } else {
                break;
            }
        }

        if !res.is_empty() {
            Some(res)
        } else {
            None
        }
    }
}

impl<I> Iterator for Lexer<I> where I: Iterator<Item=char> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.it.peek().is_some() {
            if let Some(s) = self.take_while_is_not_hangeul() {
                Some(Token::Any(s))
            } else if let Some(s) = self.take_while_is_hangeul() {
                Some(Token::Word(Word::new(s)))
            } else {
                None
            }
        } else {
            None
        }
    }
}
