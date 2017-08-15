mod lexer;
mod revised_romanization;
mod pronunciation;
mod syllable;
mod word;

use lexer::{Lexer, Token};
pub use syllable::Syllable;
pub use word::Word;

pub fn romanize(input: &str) -> String {
    let lexer = Lexer::new(input.chars());
    let mut res = String::new();

    for token in lexer {
        match token {
            Token::Word(word) => res.push_str(&word.romanize()),
            Token::Any(s) => res.push_str(&s),
        }
    }

    res
}
