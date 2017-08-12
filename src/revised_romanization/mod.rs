mod rules;
mod jamo;

use ::Word;
use self::rules::*;
use self::jamo::*;

pub fn transcribe(word: &Word) -> String {
    let syllables = word.syllables();
    let mut it = syllables.iter().peekable();
    let mut res = String::new();

    while let Some(s) = it.next() {
        let (j, k, m) = s.decompose();

        res.push_str(transliterate_initial_consonant(j));
        res.push_str(transliterate_medial_vowel(k));

        if let Some(m) = m {
            /* let t = it.peek();
            let n = t.map(|t| t.choseong()); */
            res.push_str(transliterate_final_consonant(m));
        }
    }

    res
}

pub fn pronounce(word: &Word) -> String {
    let syllables = word.syllables();
    let mut it = syllables.iter().peekable();
    let mut res = String::new();

    let mut s = it.next().map(|t| *t);
    let mut t = it.next().map(|t| *t);

    while let Some(u) = s {
        let v = t;

        let (u, v) = rule_17(u, v);

        let u = rule_05_1(u);
        let u = rule_05_3(u);
        let u = rule_09(u, v);
        let u = rule_10(u, v);
        let u = rule_11(u, v);
        let u = rule_11(u, v);
        let (u, v) = rule_14(u, v);
        let (u, v) = rule_23(u, v);

        let (u, v) = rule_13(u, v);

        res.push(u.as_char());

        s = v;
        t = it.next().map(|t| *t);
    }

    res
}

#[cfg(test)]
mod tests {
    use ::Word;

    use super::pronounce;

    fn t(s: &str) -> String {
        let word = Word::new(s);
        pronounce(&word)
    }

    #[test]
    fn test_rule_05_1() {
        assert_eq!(t("가져"), "가저");
        assert_eq!(t("쪄"), "쩌");
        assert_eq!(t("다쳐"), "다처");
    }

    #[test]
    fn test_rule_09() {
        assert_eq!(t("닦다"), "닥따");
        assert_eq!(t("키읔"), "키윽");
        assert_eq!(t("키읔과"), "키윽꽈");
        assert_eq!(t("옷"), "옫");
        assert_eq!(t("웃다"), "욷따");
        assert_eq!(t("있다"), "읻따");
        assert_eq!(t("젖"), "젇");
        assert_eq!(t("빚다"), "빋따");
        assert_eq!(t("꽃"), "꼳");
        assert_eq!(t("쫓다"), "쫃따");
        assert_eq!(t("솥"), "솓");
        assert_eq!(t("뱉다"), "밷따");
        assert_eq!(t("앞"), "압");
        assert_eq!(t("덮다"), "덥따");
    }

    #[test]
    fn test_rule_10() {
        assert_eq!(t("넋"), "넉");
        assert_eq!(t("넋과"), "넉꽈");
        // assert_eq!(t("앉다"), "안따");
        assert_eq!(t("여덟"), "여덜");
        // assert_eq!(t("넓다"), "널따");
        assert_eq!(t("외곬"), "외골");
        // assert_eq!(t("핥다"), "할따");
        assert_eq!(t("값"), "갑");
        assert_eq!(t("없다"), "업따");
    }

    #[test]
    fn test_rule_11() {
        assert_eq!(t("닭"), "닥");
        assert_eq!(t("흙과"), "흑꽈");
        assert_eq!(t("맑다"), "막따");
        assert_eq!(t("늙지"), "늑찌");
        assert_eq!(t("삶"), "삼");
        // assert_eq!(t("젊다"), "점따");
        assert_eq!(t("읊고"), "읍꼬");
        assert_eq!(t("읊다"), "읍따");
    }

    #[test]
    fn test_rule_13() {
        assert_eq!(t("깎아"), "까까");
        assert_eq!(t("옷이"), "오시");
        assert_eq!(t("있어"), "이써");
        assert_eq!(t("낮이"), "나지");
        assert_eq!(t("꽂아"), "꼬자");
        assert_eq!(t("꽃을"), "꼬츨");
        assert_eq!(t("쫓아"), "쪼차");
        assert_eq!(t("밭에"), "바테");
        assert_eq!(t("앞으로"), "아프로");
        assert_eq!(t("덮이다"), "더피다");
    }

    #[test]
    fn test_rule_14() {
		assert_eq!(t("넋이"), "넉씨");
		assert_eq!(t("앉아"), "안자");
		assert_eq!(t("닭을"), "달글");
		assert_eq!(t("젊어"), "절머");
		assert_eq!(t("곬이"), "골씨");
		assert_eq!(t("핥아"), "할타");
		assert_eq!(t("읊어"), "을퍼");
		assert_eq!(t("값을"), "갑쓸");
		assert_eq!(t("없어"), "업써");
    }

    #[test]
    fn test_rule_17() {
		assert_eq!(t("곧이듣다"), "고지듣따");
		assert_eq!(t("굳이"), "구지");
		assert_eq!(t("미닫이"), "미다지");
		assert_eq!(t("땀받이"), "땀바지");
		assert_eq!(t("밭이"), "바치");
		assert_eq!(t("벼훑이"), "벼훌치");
    }

    #[test]
    fn test_rule_23() {
        assert_eq!(t("국밥"), "국빱");
        assert_eq!(t("깎다"), "깍따");
        assert_eq!(t("넋받이"), "넉빠지");
        assert_eq!(t("삯돈"), "삭똔");
        assert_eq!(t("닭장"), "닥짱");
        assert_eq!(t("칡범"), "칙뻠");
        assert_eq!(t("뻗대다"), "뻗때다");
        assert_eq!(t("옷고름"), "옫꼬름");
        assert_eq!(t("있던"), "읻떤");
        assert_eq!(t("꽂고"), "꼳꼬");
        assert_eq!(t("꽃다발"), "꼳따발");
        assert_eq!(t("낯설다"), "낟썰다");
        assert_eq!(t("밭갈이"), "받까리");
        assert_eq!(t("솥전"), "솓쩐");
        assert_eq!(t("곱돌"), "곱똘");
        assert_eq!(t("덮개"), "덥깨");
        assert_eq!(t("옆집"), "엽찝");
        // assert_eq!(t("넓죽하다"), "넙쭈카다");
        assert_eq!(t("읊조리다"), "읍쪼리다");
        assert_eq!(t("값지다"), "갑찌다");
    }
}
