use ::Syllable;
use revised_romanization::{pronounce, transcribe};

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
    ///     Syllable::new('좋').unwrap(),
    ///     Syllable::new('아').unwrap(),
    ///     Syllable::new('요').unwrap(),
    /// ]);
    /// ```
    pub fn syllables(&self) -> Vec<Syllable> {
        self.0.chars().filter_map(|c| Syllable::new(c).ok()).collect()
    }

    pub fn romanize(&self) -> String {
        let word = Word::new(pronounce(self));
        transcribe(&word)
    }
}
