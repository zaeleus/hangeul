use ::Syllable;
use pronunciation::Pronouncer;

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

pub fn rule_12_1(mut s: Syllable, t: Option<Syllable>) -> (Syllable, Option<Syllable>) {
    if let Some(j) = s.jongseong() {
        if let Some(mut t) = t {
            let km = match j {
                'ㅎ' => match t.choseong() {
                    'ㄱ' => Some((None, 'ㅋ')),
                    'ㄷ' => Some((None, 'ㅌ')),
                    'ㅈ' => Some((None, 'ㅊ')),
                    _ => None,
                },
                'ㄶ' => match t.choseong() {
                    'ㄱ' => Some((Some('ㄴ'), 'ㅋ')),
                    'ㄷ' => Some((Some('ㄴ'), 'ㅌ')),
                    'ㅈ' => Some((Some('ㄴ'), 'ㅊ')),
                    _ => None,
                },
                'ㅀ' => match t.choseong() {
                    'ㄱ' => Some((Some('ㄹ'), 'ㅋ')),
                    'ㄷ' => Some((Some('ㄹ'), 'ㅌ')),
                    'ㅈ' => Some((Some('ㄹ'), 'ㅊ')),
                    _ => None,
                },
                _ => None,
            };

            if let Some((k, m)) = km {
                s.set_jongseong(k);
                t.set_choseong(m);
                return (s, Some(t));
            }
        }
    }

    (s, t)
}


pub fn rule_12_1_1(mut s: Syllable, t: Option<Syllable>) -> (Syllable, Option<Syllable>) {
    if let Some(j) = s.jongseong() {
        if let Some(mut t) = t {
            if t.choseong() == 'ㅎ' {
                let km = match j {
                    'ㄱ' => Some((None, 'ㅋ')),
                    'ㄺ' => Some((Some('ㄹ'), 'ㅋ')),
                    'ㄷ' => Some((None, 'ㅌ')),
                    'ㅂ' => Some((None, 'ㅍ')),
                    'ㄼ' => Some((Some('ㄹ'), 'ㅍ')),
                    'ㅈ' => Some((None, 'ㅊ')),
                    'ㄵ' => Some((Some('ㄴ'), 'ㅊ')),
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

pub fn rule_12_1_2(mut s: Syllable, t: Option<Syllable>) -> (Syllable, Option<Syllable>) {
    if let Some(j) = s.jongseong() {
        if let Some(mut t) = t {
            if t.choseong() == 'ㅎ' {
                match j {
                    'ㅅ' | /* 'ㅈ' | */ 'ㅊ' | 'ㅌ' => {
                        s.set_jongseong(None);
                        t.set_choseong('ㅌ');
                        return (s, Some(t));
                    },
                    _ => {}
                };
            }
        }
    }

    (s, t)
}

pub fn rule_12_2(mut s: Syllable, t: Option<Syllable>) -> (Syllable, Option<Syllable>) {
    if let Some(j) = s.jongseong() {
        if let Some(mut t) = t {
            if t.choseong() == 'ㅅ' {
                let k = match j {
                    'ㅎ' => Some(None),
                    'ㄶ' => Some(Some('ㄴ')),
                    'ㅀ' => Some(Some('ㄹ')),
                    _ => None,
                };

                if let Some(k) = k {
                    s.set_jongseong(k);
                    t.set_choseong('ㅆ');
                    return (s, Some(t));
                }
            }
        }
    }

    (s, t)
}

pub fn rule_12_3(mut s: Syllable, t: Option<Syllable>) -> (Syllable, Option<Syllable>) {
    if let Some(j) = s.jongseong() {
        if let Some(t) = t {
            if t.choseong() == 'ㄴ' {
                let k = match j {
                    'ㅎ' => Some('ㄴ'),
                    'ㄶ' => Some('ㄴ'),
                    'ㅀ' => Some('ㄹ'),
                    _ => None,
                };

                if k.is_some() {
                    s.set_jongseong(k);
                    return (s, Some(t));
                }
            }
        }
    }

    (s, t)
}

pub fn rule_12_4(mut s: Syllable, t: Option<Syllable>) -> (Syllable, Option<Syllable>) {
    if let Some(j) = s.jongseong() {
        if let Some(mut t) = t {
            if t.choseong() == 'ㅇ' {
                let k = match j {
                    'ㅎ' => Some('ㅇ'),
                    'ㄶ' => Some('ㄴ'),
                    'ㅀ' => Some('ㄹ'),
                    _ => None,
                };


                if let Some(k) = k {
                    s.set_jongseong(None);
                    t.set_choseong(k);
                    return (s, Some(t));
                }
            }
        }
    }

    (s, t)
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

pub fn rule_14(ctx: &Pronouncer, mut s: Syllable, t: Option<Syllable>) -> (Syllable, Option<Syllable>) {
    if let Some(j) = s.jongseong() {
        if let Some(mut t) = t {
            if t.choseong() == 'ㅇ' {
                let km = match j {
                    'ㄳ' => Some(('ㄱ', ctx.reflect_tense('ㅅ'))),
                    'ㄵ' => Some(('ㄴ', 'ㅈ')),
                    'ㄶ' => Some(('ㄴ', 'ㅎ')),
                    'ㄺ' => Some(('ㄹ', 'ㄱ')),
                    'ㄻ' => Some(('ㄹ', 'ㅁ')),
                    'ㄼ' => Some(('ㄹ', 'ㅂ')),
                    'ㄽ' => Some(('ㄹ', ctx.reflect_tense('ㅅ'))),
                    'ㄾ' => Some(('ㄹ', 'ㅌ')),
                    'ㄿ' => Some(('ㄹ', 'ㅍ')),
                    'ㅀ' => Some(('ㄹ', 'ㅎ')),
                    'ㅄ' => Some(('ㅂ', ctx.reflect_tense('ㅅ'))),
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

pub fn rule_16(s: Syllable, t: Option<Syllable>) -> Option<Syllable> {
    if let Some(mut t) = t {
        let j = match (s.as_char(), t.as_char()) {
            ('디', '귿') => Some('ㅅ'),
            ('지', '읒') => Some('ㅅ'),
            ('치', '읓') => Some('ㅅ'),
            ('키', '읔') => Some('ㄱ'),
            ('티', '읕') => Some('ㅅ'),
            ('피', '읖') => Some('ㅂ'),
            ('히', '읗') => Some('ㅅ'),
            _ => None,
        };

        if j.is_some() {
            t.set_jongseong(j);
            return Some(t);
        }
    }

    t
}

pub fn rule_17(mut s: Syllable, t: Option<Syllable>) -> (Syllable, Option<Syllable>) {
    if let Some(j) = s.jongseong() {
        if let Some(mut t) = t {
            let km = match (t.choseong(), t.jungseong()) {
                ('ㅇ', 'ㅣ') => match j {
                    'ㄷ' => Some((None, 'ㅈ')),
                    'ㅌ' => Some((None, 'ㅊ')),
                    'ㄾ' => Some((Some('ㄹ'), 'ㅊ')),
                    _ => None,
                },
                ('ㅎ', 'ㅣ') => match j {
                    'ㄷ' => Some((None, 'ㅊ')),
                    _ => None,
                },
                _ => None,
            };

            if let Some((k, m)) = km {
                s.set_jongseong(k);
                t.set_choseong(m);
                return (s, Some(t));
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

pub fn rule_19(mut s: Syllable, t: Option<Syllable>) -> (Syllable, Option<Syllable>) {
    if let Some(j) = s.jongseong() {
        if let Some(mut t) = t {
            if t.choseong() == 'ㄹ' {
                let k = match j {
                    'ㅁ' | 'ㅇ' => Some(j),
                    'ㄱ' => Some('ㅇ'),
                    'ㅂ' => Some('ㅁ'),
                    _ => None
                };

                if k.is_some() {
                    s.set_jongseong(k);
                    t.set_choseong('ㄴ');
                    return (s, Some(t));
                }
            }
        }
    }

    (s, t)
}

pub fn rule_20(mut s: Syllable, t: Option<Syllable>) -> (Syllable, Option<Syllable>) {
    if let Some(j) = s.jongseong() {
        if let Some(mut t) = t {
            let k = t.choseong();

            if (j == 'ㄴ' && k == 'ㄹ') || (j == 'ㄹ' && k == 'ㄴ') {
                s.set_jongseong(Some('ㄹ'));
                t.set_choseong('ㄹ');
                return (s, Some(t));
            }
        }
    }

    (s, t)
}

pub fn rule_23(ctx: &Pronouncer, s: Syllable, t: Option<Syllable>) -> (Syllable, Option<Syllable>) {
    if let Some(j) = s.jongseong() {
        if let Some(mut t) = t {
            let k = t.choseong();

            let m = match j {
                'ㄱ' | 'ㄲ' | 'ㅋ' | 'ㄳ'
                        | 'ㄺ' | 'ㄷ' | 'ㅅ' | 'ㅆ'
                        | 'ㅈ' | 'ㅊ' | 'ㅌ' | 'ㅂ'
                        | 'ㅍ' | 'ㄼ' | 'ㄿ' | 'ㅄ' => match k {
                    'ㄱ' | 'ㄷ' | 'ㅂ' | 'ㅅ' | 'ㅈ' => Some(ctx.reflect_tense(k)),
                    _ => None,
                },
                _ => None,
            };

            if let Some(m) = m {
                t.set_choseong(m);
                return (s, Some(t));
            }
        }
    }

    (s, t)
}

pub fn rule_24(ctx: &Pronouncer, mut s: Syllable, t: Option<Syllable>) -> (Syllable, Option<Syllable>) {
    if let Some(j) = s.jongseong() {
        if let Some(mut t) = t {
            let k = t.choseong();

            let km = match k {
                'ㄱ' | 'ㄷ' | 'ㅅ' | 'ㅈ' => match j {
                    'ㄴ' | 'ㅁ' => Some((j, ctx.reflect_tense(k))),
                    'ㄵ' => Some(('ㄴ', ctx.reflect_tense(k))),
                    'ㄻ' => Some(('ㅁ', ctx.reflect_tense(k))),
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

pub fn rule_25(ctx: &Pronouncer, mut s: Syllable, t: Option<Syllable>) -> (Syllable, Option<Syllable>) {
    if let Some(j) = s.jongseong() {
        if let Some(mut t) = t {
            let k = t.choseong();

            let m = match j {
                'ㄼ' | 'ㄾ' => match k {
                    'ㄱ' | 'ㄷ' | 'ㅅ' | 'ㅈ' => Some(ctx.reflect_tense(k)),
                    _ => None,
                },
                _ => None,
            };

            if let Some(m) = m {
                s.set_jongseong(Some('ㄹ'));
                t.set_choseong(m);
                return (s, Some(t));
            }
        }
    }

    (s, t)
}
