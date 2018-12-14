use crate::Syllable;
use crate::pronunciation::Pronouncer;
use crate::revised_romanization::transcribe;

/// A word composed of Hangeul syllables
#[derive(Debug)]
pub struct Word(String);

impl Word {
    /// Wraps a string of Hangeul characters.
    ///
    /// Strings must only contain Hangeul syllables.
    ///
    /// # Examples
    ///
    /// ```
    /// use hangeul::Word;
    /// let _ = Word::new("몰라요");
    /// ```
    pub fn new<S>(s: S) -> Word where S: Into<String> {
        Word(s.into())
    }

    /// Returns a list of syllables that make up the word.
    ///
    /// # Examples
    ///
    /// ```
    /// use hangeul::{Syllable, Word};
    ///
    /// let word = Word::new("좋아요");
    /// assert_eq!(word.syllables(), vec![
    ///     Syllable::from_char('좋').unwrap(),
    ///     Syllable::from_char('아').unwrap(),
    ///     Syllable::from_char('요').unwrap(),
    /// ]);
    /// ```
    pub fn syllables(&self) -> Vec<Syllable> {
        self.0.chars().filter_map(|c| Syllable::from_char(c).ok()).collect()
    }

    pub fn romanize(&self) -> String {
        let pronouncer = Pronouncer::new(false);
        let transformed_word = pronouncer.pronounce(self);
        transcribe(&transformed_word)
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}
