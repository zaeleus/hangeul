mod jamo;

use ::Word;
use self::jamo::*;

pub fn transcribe(word: &Word) -> String {
    let syllables = word.syllables();
    let mut it = syllables.iter().peekable();
    let mut res = String::new();

    let mut skip = false;

    while let Some(s) = it.next() {
        let (j, k, m) = s.decompose();

        if !skip {
            res.push_str(transliterate_initial_consonant(j));
        } else {
            skip = false;
        }

        res.push_str(transliterate_medial_vowel(k));

        if let Some(m) = m {
            let t = it.peek();

            if let Some(n) = t.map(|t| t.choseong()) {
                if m == 'ㄹ' && n == 'ㄹ' {
                    res.push_str("ll");
                    skip = true;
                    continue;
                }
            }

            res.push_str(transliterate_final_consonant(m));
        }
    }

    res
}
