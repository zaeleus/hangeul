use std::char;

// @see http://www.unicode.org/charts/PDF/UAC00.pdf
// @see http://www.w3c.or.kr/i18n/hangul-i18n/ko-code.html
const HANGEUL_SYLLABLE_OFFSET: u32 = 0xac00;
const HANGEUL_SYLLABLE_SIZE: u32 = 19 * 21 * 28;

static CHOSEONGS: &'static [char] = &[
    'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ',
    'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ',
    'ㅌ', 'ㅍ', 'ㅎ',
];

static JUNGSEONGS: &'static [char] = &[
    'ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ',
    'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ', 'ㅞ',
    'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ',
];

static JONGSEONGS: &'static [char] = &[
    'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ',
    'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ', 'ㄿ', 'ㅀ', 'ㅁ',
    'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ',
    'ㅌ', 'ㅍ', 'ㅎ',
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
/// [UAC00.pdf]: http://www.unicode.org/charts/PDF/UAC00.pdf
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Syllable(char, char, Option<char>);

impl Syllable {
    pub fn compose(j: char, k: char, m: Option<char>) -> Option<char> {
        let ij = CHOSEONGS.iter().position(|&n| n == j);
        let ik = JUNGSEONGS.iter().position(|&n| n == k);

        if let Some(ij) = ij {
            if let Some(ik) = ik {
                let im = m
                    .and_then(|m| JONGSEONGS.iter().position(|&n| n == m))
                    .map(|i| i + 1)
                    .unwrap_or(0);

                let offset = HANGEUL_SYLLABLE_OFFSET as usize;
                let c = offset + (ij * 21 * 28) + (ik * 28) + im;

                return char::from_u32(c as u32);
            }
        }

        None
    }

    /// Validates whether a character is a Hangeul syllable (0xac00-0xd7a3).
    pub fn is_valid(s: char) -> bool {
        let u = s as u32;
        u >= HANGEUL_SYLLABLE_OFFSET
            && u < HANGEUL_SYLLABLE_OFFSET + HANGEUL_SYLLABLE_SIZE
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
    /// assert!(Syllable::new('이').is_ok());
    /// assert!(Syllable::new('달').is_ok());
    /// assert!(Syllable::new('a').is_err());
    /// assert!(Syllable::new('あ').is_err());
    /// ```
    pub fn new(s: char) -> Result<Syllable, Error> {
        if !Syllable::is_valid(s) {
            return Err(Error::InvalidSyllable);
        }

        let i = extract_choseong(s).unwrap();
        let j = extract_jungseong(s).unwrap();
        let m = extract_jongseong(s).unwrap();

        Ok(Syllable(i, j, m))
    }

    pub fn choseong(&self) -> char {
        self.0
    }

    pub fn set_choseong(&mut self, c: char) {
        self.0 = c;
    }

    pub fn jungseong(&self) -> char {
        self.1
    }

    pub fn set_jungseong(&mut self, c: char)  {
        self.1 = c;
    }

    pub fn jongseong(&self) -> Option<char> {
        self.2
    }

    pub fn set_jongseong(&mut self, c: Option<char>) {
        self.2 = c;
    }

    pub fn decompose(&self) -> (char, char, Option<char>) {
        (self.choseong(), self.jungseong(), self.jongseong())
    }

    pub fn as_char(&self) -> char {
        Syllable::compose(self.choseong(), self.jungseong(), self.jongseong()).unwrap()
    }
}

fn extract_choseong(s: char) -> Result<char, Error> {
    let u = s as u32;
    let i = ((u - HANGEUL_SYLLABLE_OFFSET) / (21 * 28)) as usize;
    CHOSEONGS.get(i).map_or(Err(Error::Indecomposable), |&c| Ok(c))
}

fn extract_jungseong(s: char) -> Result<char, Error> {
    let u = s as u32;
    let i = (((u - HANGEUL_SYLLABLE_OFFSET) % (21 * 28)) / 28) as usize;
    JUNGSEONGS.get(i).map_or(Err(Error::Indecomposable), |&c| Ok(c))
}

fn extract_jongseong(s: char) -> Result<Option<char>, Error> {
    let u = s as i32;
    let i = (u - HANGEUL_SYLLABLE_OFFSET as i32) % 28 - 1;

    if i >= 0 {
        JONGSEONGS.get(i as usize).map_or(Err(Error::Indecomposable), |&c| {
            Ok(Some(c))
        })
    } else {
        Ok(None)
    }
}
