use ::Syllable;

pub fn rule_05_1(mut s: Syllable) -> Syllable {
    match s.choseong() {
        'ㅈ' | 'ㅉ' | 'ㅊ' => if s.jungseong() == 'ㅕ' {
            s.set_jungseong('ㅓ');
        },
        _ => {},
    }

    s
}

pub fn rule_05_3(mut s: Syllable) -> Syllable {
    if s.choseong() != 'ㅇ' && s.jungseong() == 'ㅢ' {
        s.set_jungseong('ㅣ');
    }

    s
}

pub fn rule_09(mut s: Syllable, t: Option<Syllable>) -> Syllable {
    if let Some(j) = s.jongseong() {
        if t.map_or(true, |t| t.choseong() != 'ㅇ') {
            let k = match j {
                'ㄲ' | 'ㅋ' => Some('ㄱ'),
                'ㅅ' | 'ㅆ' | 'ㅈ' | 'ㅊ' | 'ㅌ' => Some('ㄷ'),
                'ㅍ' => Some('ㅂ'),
                _ => None,
            };

            if k.is_some() {
                s.set_jongseong(k);
            }
        }
    }

    s
}

pub fn rule_10(mut s: Syllable, t: Option<Syllable>) -> Syllable {
    if let Some(j) = s.jongseong() {
        if t.map_or(true, |t| t.choseong() != 'ㅇ') {
            let k = match j {
                'ㄳ' => Some('ㄱ'),
                'ㄵ' => Some('ㄴ'),
                'ㄼ' | 'ㄽ' | 'ㄾ' => Some('ㄹ'),
                'ㅄ' => Some('ㅂ'),
                _ => None,
            };

            if k.is_some() {
                s.set_jongseong(k);
            }
        }
    }

    s
}

pub fn rule_11(mut s: Syllable, t: Option<Syllable>) -> Syllable {
    if let Some(j) = s.jongseong() {
        if t.map_or(true, |t| t.choseong() != 'ㅇ') {
            let k = match j {
                'ㄺ' => Some('ㄱ'),
                'ㄻ' => Some('ㅁ'),
                'ㄿ' => Some('ㅂ'),
                _ => None,
            };

            if k.is_some() {
                s.set_jongseong(k);
            }
        }
    }

    s
}

pub fn rule_13(mut s: Syllable, t: Option<Syllable>) -> (Syllable, Option<Syllable>) {
    if let Some(j) = s.jongseong() {
        if j != 'ㅇ' {
            if let Some(mut t) = t {
                if t.choseong() == 'ㅇ' {
                    s.set_jongseong(None);
                    t.set_choseong(j);
                    return (s, Some(t));
                }
            }
        }
    }

    (s, t)
}

pub fn rule_14(mut s: Syllable, t: Option<Syllable>) -> (Syllable, Option<Syllable>) {
    if let Some(j) = s.jongseong() {
        if let Some(mut t) = t {
            if t.choseong() == 'ㅇ' {
                let km = match j {
                    'ㄳ' => Some(('ㄱ', 'ㅆ')),
                    'ㄵ' => Some(('ㄴ', 'ㅈ')),
                    'ㄶ' => Some(('ㄴ', 'ㅎ')),
                    'ㄺ' => Some(('ㄹ', 'ㄱ')),
                    'ㄻ' => Some(('ㄹ', 'ㅁ')),
                    'ㄼ' => Some(('ㄹ', 'ㅂ')),
                    'ㄽ' => Some(('ㄹ', 'ㅆ')),
                    'ㄾ' => Some(('ㄹ', 'ㅌ')),
                    'ㄿ' => Some(('ㄹ', 'ㅍ')),
                    'ㅀ' => Some(('ㄹ', 'ㅎ')),
                    'ㅄ' => Some(('ㅂ', 'ㅆ')),
                    _ => None,
                };

                if let Some((k, m)) = km {
                    s.set_jongseong(Some(k));
                    t.set_choseong(m);
                    return (s, Some(t));
                }
            }
        }
    }

    (s, t)
}

pub fn rule_17(mut s: Syllable, t: Option<Syllable>) -> (Syllable, Option<Syllable>) {
    if let Some(j) = s.jongseong() {
        if let Some(mut t) = t {
            if t.choseong() == 'ㅇ' && t.jungseong() == 'ㅣ' {
                let km = match j {
                    'ㄷ' => Some((None, 'ㅈ')),
                    'ㅌ' => Some((None, 'ㅊ')),
                    'ㄾ' => Some((Some('ㄹ'), 'ㅊ')),
                    _ => None,
                };

                if let Some((k, m)) = km {
                    s.set_jongseong(k);
                    t.set_choseong(m);
                    return (s, Some(t));
                }
            }
        }
    }

    (s, t)
}

pub fn rule_18(mut s: Syllable, t: Option<Syllable>) -> Syllable {
    if let Some(j) = s.jongseong() {
        if t.map_or(false, |t| t.choseong() == 'ㄴ' || t.choseong() == 'ㅁ') {
            let k = match j {
                'ㄱ' | 'ㄲ' | 'ㅋ' | 'ㄳ' | 'ㄺ' => Some('ㅇ'),
                'ㄷ' | 'ㅅ' | 'ㅆ' | 'ㅈ' | 'ㅊ' | 'ㅌ' | 'ㅎ' => Some('ㄴ'),
                'ㅂ' | 'ㅍ' | 'ㄼ' | 'ㄿ' | 'ㅄ' => Some('ㅁ'),
                _ => None,
            };

            if k.is_some() {
                s.set_jongseong(k);
            }
        }
    }

    s
}

pub fn rule_19(s: Syllable, t: Option<Syllable>) -> (Syllable, Option<Syllable>) {
    if let Some(j) = s.jongseong() {
        if let Some(mut t) = t {
            if t.choseong() == 'ㄹ' {
                if j == 'ㅁ' || j == 'ㅇ' {
                    t.set_choseong('ㄴ');
                    return (s, Some(t));
                }
            }
        }
    }

    (s, t)
}

pub fn rule_23(mut s: Syllable, t: Option<Syllable>) -> (Syllable, Option<Syllable>) {
    if let Some(j) = s.jongseong() {
        if let Some(mut t) = t {
            let km = match j {
                'ㄱ' | 'ㄷ' | 'ㅂ' => match t.choseong() {
                    'ㄱ' => Some((j, 'ㄲ')),
                    'ㄷ' => Some((j, 'ㄸ')),
                    'ㅂ' => Some((j, 'ㅃ')),
                    'ㅅ' => Some((j, 'ㅆ')),
                    'ㅈ' => Some((j, 'ㅉ')),
                    _ => None,
                },
                _ => None,
            };

            if let Some((k, m)) = km {
                s.set_jongseong(Some(k));
                t.set_choseong(m);
                return (s, Some(t));
            }
        }
    }

    (s, t)
}
