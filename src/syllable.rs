use std::char;

// @see http://www.unicode.org/charts/PDF/UAC00.pdf
// @see http://www.w3c.or.kr/i18n/hangul-i18n/ko-code.html
const SYLLABLES_OFFSET: usize = 0xac00;
const SYLLABLES_LEN: usize = 19 * 21 * 28;

// @see https://www.unicode.org/charts/PDF/U1100.pdf
const CHOSEONGS_OFFSET: usize = 0x1100;
const CHOSEONGS_LEN: usize = 19;
static CHOSEONGS: [char; CHOSEONGS_LEN] = [
    'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ',
    'ㅉ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
];

const JUNGSEONGS_OFFSET: usize = 0x1161;
const JUNGSEONGS_LEN: usize = 21;
static JUNGSEONGS: [char; JUNGSEONGS_LEN] = [
    'ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ',
    'ㅜ', 'ㅝ', 'ㅞ', 'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ',
];

const JONGSEONGS_OFFSET: usize = 0x11a8;
const JONGSEONGS_LEN: usize = 27;
static JONGSEONGS: [char; JONGSEONGS_LEN] = [
    'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ',
    'ㄿ', 'ㅀ', 'ㅁ', 'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ',
    'ㅎ',
];

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    Indecomposable,
    InvalidSyllable,
}

/// A Hangeul syllable
///
/// A syllable is any precomposed cluster of Hangeul jamos (letters). 11184
/// characters are defined in [the 0xAC00-0xD7AF Unicode range][UAC00.pdf].
///
/// An instance of `Syllable` is guaranteed to be composed of valid jamos.
///
/// [UAC00.pdf]: http://www.unicode.org/charts/PDF/UAC00.pdf
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Syllable(char, char, Option<char>);

impl Syllable {
    /// Creates a syllable from the decomposed jamos of a syllable.
    ///
    /// # Examples
    ///
    /// ```
    /// use hangeul::Syllable;
    ///
    /// assert_eq!(Syllable::compose('ㄱ', 'ㅣ', Some('ㅁ')), Some('김'));
    /// assert_eq!(Syllable::compose('ㅅ', 'ㅔ', None), Some('세'));
    /// assert_eq!(Syllable::compose('ㅈ', 'ㅓ', Some('ㅇ')), Some('정'));
    /// assert_eq!(Syllable::compose('a', 'b', Some('c')), None);
    /// ```
    pub fn compose(j: char, k: char, m: Option<char>) -> Option<char> {
        let ij = CHOSEONGS.iter().position(|&n| n == j);
        let ik = JUNGSEONGS.iter().position(|&n| n == k);

        if let Some(ij) = ij {
            if let Some(ik) = ik {
                let im = m
                    .and_then(|m| JONGSEONGS.iter().position(|&n| n == m))
                    .map(|i| i + 1)
                    .unwrap_or(0);

                let c = SYLLABLES_OFFSET + (ij * 21 * 28) + (ik * 28) + im;

                return char::from_u32(c as u32);
            }
        }

        None
    }

    /// Validates whether a character is a Hangeul syllable (0xac00-0xd7a3).
    ///
    /// # Examples
    ///
    /// ```
    /// use hangeul::Syllable;
    ///
    /// assert!(Syllable::is_valid('이'));
    /// assert!(Syllable::is_valid('달'));
    /// assert!(!Syllable::is_valid('a'));
    /// assert!(!Syllable::is_valid('あ'));
    /// ```
    pub fn is_valid(s: char) -> bool {
        let u = s as usize;
        is_between(u, SYLLABLES_OFFSET, SYLLABLES_OFFSET + SYLLABLES_LEN)
    }

    /// Wraps a Hangeul character for decomposition.
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidSyllable` if the given character is not a valid
    /// Hangeul syllable (0xac00-0xd7a3).
    ///
    /// # Examples
    ///
    /// ```
    /// use hangeul::Syllable;
    ///
    /// assert!(Syllable::from_char('이').is_ok());
    /// assert!(Syllable::from_char('달').is_ok());
    /// assert!(Syllable::from_char('a').is_err());
    /// assert!(Syllable::from_char('あ').is_err());
    /// ```
    pub fn from_char(c: char) -> Result<Syllable, Error> {
        let j = extract_choseong(c)?;
        let k = extract_jungseong(c)?;
        let m = extract_jongseong(c)?;
        Ok(Syllable::new(j, k, m))
    }

    /// Creates a new syllable from jamos.
    ///
    /// Note this does not validate the resulting `Syllable`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hangeul::Syllable;
    ///
    /// let s = Syllable::new('ㅇ', 'ㅣ', None);
    /// assert_eq!(s.choseong(), 'ㅇ');
    /// assert_eq!(s.jungseong(), 'ㅣ');
    /// assert_eq!(s.jongseong(), None);
    ///
    /// let s = Syllable::new('ㄷ', 'ㅏ', Some('ㄹ'));
    /// assert_eq!(s.choseong(), 'ㄷ');
    /// assert_eq!(s.jungseong(), 'ㅏ');
    /// assert_eq!(s.jongseong(), Some('ㄹ'));
    /// ```

    pub fn new(j: char, k: char, m: Option<char>) -> Syllable {
        Syllable(j, k, m)
    }

    pub fn choseong(&self) -> char {
        self.0
    }

    /// Sets the initial consonant.
    ///
    /// # Panics
    ///
    /// Panics when the given character is not a valid choseong.
    pub fn set_choseong(&mut self, c: char) {
        let u = c as usize;

        if is_between(u, CHOSEONGS_OFFSET, CHOSEONGS_OFFSET + CHOSEONGS_LEN) {
            panic!("invalid choseong '{}'", c);
        }

        self.0 = c;
    }

    pub fn jungseong(&self) -> char {
        self.1
    }

    /// Sets the medial vowel.
    ///
    /// # Panics
    ///
    /// Panics when the given character is not a valid jungseong.
    pub fn set_jungseong(&mut self, c: char) {
        let u = c as usize;

        if is_between(u, JUNGSEONGS_OFFSET, JUNGSEONGS_OFFSET + JUNGSEONGS_LEN) {
            panic!("invalid junseong '{}'", c);
        }

        self.1 = c;
    }

    pub fn jongseong(&self) -> Option<char> {
        self.2
    }

    /// Sets the optional final consonant.
    ///
    /// # Panics
    ///
    /// Panics when the given character is not a valid jongseong.
    pub fn set_jongseong(&mut self, c: Option<char>) {
        if let Some(d) = c {
            let u = d as usize;

            if is_between(u, JONGSEONGS_OFFSET, JONGSEONGS_OFFSET + JONGSEONGS_LEN) {
                panic!("invalid jonseong '{}'", d);
            }
        }

        self.2 = c;
    }

    pub fn decompose(&self) -> (char, char, Option<char>) {
        (self.choseong(), self.jungseong(), self.jongseong())
    }

    pub fn as_char(&self) -> char {
        Syllable::compose(self.choseong(), self.jungseong(), self.jongseong()).unwrap()
    }
}

/// Returns whether `n` is in [`lower`, `upper`).
fn is_between(n: usize, lower: usize, upper: usize) -> bool {
    n >= lower && n < upper
}

fn extract_choseong(s: char) -> Result<char, Error> {
    if !Syllable::is_valid(s) {
        return Err(Error::InvalidSyllable);
    }

    let u = s as usize;
    let i = (u - SYLLABLES_OFFSET) / (21 * 28);
    CHOSEONGS
        .get(i)
        .map_or(Err(Error::Indecomposable), |&c| Ok(c))
}

fn extract_jungseong(s: char) -> Result<char, Error> {
    if !Syllable::is_valid(s) {
        return Err(Error::InvalidSyllable);
    }

    let u = s as usize;
    let i = ((u - SYLLABLES_OFFSET) % (21 * 28)) / 28;
    JUNGSEONGS
        .get(i)
        .map_or(Err(Error::Indecomposable), |&c| Ok(c))
}

fn extract_jongseong(s: char) -> Result<Option<char>, Error> {
    if !Syllable::is_valid(s) {
        return Err(Error::InvalidSyllable);
    }

    let u = s as isize;
    let i = (u - SYLLABLES_OFFSET as isize) % 28 - 1;

    if i >= 0 {
        JONGSEONGS
            .get(i as usize)
            .map_or(Err(Error::Indecomposable), |&c| Ok(Some(c)))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_between() {
        assert!(is_between(0, 0, 10));
        assert!(is_between(1, 0, 10));
        assert!(is_between(5, 0, 10));
        assert!(is_between(9, 0, 10));
        assert!(!is_between(10, 0, 10));
        assert!(!is_between(11, 0, 10));
    }

    #[test]
    fn test_extract_choseong() {
        assert_eq!(extract_choseong('김'), Ok('ㄱ'));
        assert_eq!(extract_choseong('세'), Ok('ㅅ'));
        assert_eq!(extract_choseong('정'), Ok('ㅈ'));

        assert!(extract_choseong('a').is_err());
    }

    #[test]
    fn test_extract_jungseong() {
        assert_eq!(extract_jungseong('김'), Ok('ㅣ'));
        assert_eq!(extract_jungseong('세'), Ok('ㅔ'));
        assert_eq!(extract_jungseong('정'), Ok('ㅓ'));

        assert!(extract_jungseong('a').is_err());
    }

    #[test]
    fn test_extract_jongseong() {
        assert_eq!(extract_jongseong('김'), Ok(Some('ㅁ')));
        assert_eq!(extract_jongseong('세'), Ok(None));
        assert_eq!(extract_jongseong('정'), Ok(Some('ㅇ')));

        assert!(extract_jongseong('a').is_err());
    }
}
