mod lexer;
mod pronunciation;
mod revised_romanization;
mod syllable;
mod word;

use crate::lexer::{Lexer, Token};
pub use crate::syllable::Syllable;
pub use crate::word::Word;

/// Romanizes text using Revised Romanization rules.
///
/// # Examples
///
/// ```
/// use hangeul::romanize;
///
/// assert_eq!(romanize("볼빨간사춘기"), "bolppalgansachungi");
/// assert_eq!(romanize("여보세요"), "yeoboseyo");
/// assert_eq!(romanize("MOMOLAND - 뿜뿜"), "MOMOLAND - ppumppum");
/// ```
pub fn romanize(input: &str) -> String {
    Lexer::new(input.chars())
        .map(|token| match token {
            Token::Word(word) => word.romanize(),
            Token::Any(s) => s,
        })
        .collect()
}
