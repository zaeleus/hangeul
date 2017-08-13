mod rules;
mod jamo;

use ::Word;
use self::rules::*;
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

pub fn pronounce(word: &Word) -> String {
    let syllables = word.syllables();
    let mut it = syllables.iter().peekable();
    let mut res = String::new();

    let mut s = it.next().map(|t| *t);
    let mut t = it.next().map(|t| *t);

    while let Some(u) = s {
        let v = t;

        let v = rule_16(u, v);
        let (u, v) = rule_17(u, v);
        let u = rule_18(u, v);
        let (u, v) = rule_19(u, v);

        let (u, v) = rule_24(u, v);
        let (u, v) = rule_25(u, v);

        let u = rule_05_1(u);
        let u = rule_05_3(u);
        let u = rule_09(u, v);
        let u = rule_10(u, v);
        let u = rule_11(u, v);
        let (u, v) = rule_12_1(u, v);
        let (u, v) = rule_12_2(u, v);
        let (u, v) = rule_12_3(u, v);
        let (u, v) = rule_12_4(u, v);
        let (u, v) = rule_14(u, v);
        let (u, v) = rule_20(u, v);
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
    fn test_rule_12_1() {
        assert_eq!(t("놓고"), "노코");
        assert_eq!(t("좋던"), "조턴");
        assert_eq!(t("쌓지"), "싸치");
        assert_eq!(t("많고"), "만코");
        assert_eq!(t("않던"), "안턴");
        assert_eq!(t("닳지"), "달치");
    }

    #[test]
    fn test_rule_12_2() {
        assert_eq!(t("닿소"), "다쏘");
        assert_eq!(t("많소"), "만쏘");
        assert_eq!(t("싫소"), "실쏘");
    }

    #[test]
    fn test_rule_12_3() {
        assert_eq!(t("놓는"), "논는");
        assert_eq!(t("쌓네"), "싼네");

        assert_eq!(t("않네"), "안네");
        assert_eq!(t("않는"), "안는");
        assert_eq!(t("뚫네"), "뚤레");
        assert_eq!(t("뚫는"), "뚤른");
    }

    #[test]
    fn test_rule_12_4() {
        assert_eq!(t("낳은"), "나은");
        assert_eq!(t("놓아"), "노아");
        assert_eq!(t("쌓이다"), "싸이다");
        assert_eq!(t("많아"), "마나");
        assert_eq!(t("않은"), "아는");
        assert_eq!(t("닳아"), "다라");
        assert_eq!(t("싫어도"), "시러도");
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
    fn test_rule_16() {
        assert_eq!(t("디귿이"), "디그시");
        assert_eq!(t("디귿을"), "디그슬");
        assert_eq!(t("디귿에"), "디그세");
        assert_eq!(t("지읒이"), "지으시");
        assert_eq!(t("지읒을"), "지으슬");
        assert_eq!(t("지읒에"), "지으세");
        assert_eq!(t("치읓이"), "치으시");
        assert_eq!(t("치읓을"), "치으슬");
        assert_eq!(t("치읓에"), "치으세");
        assert_eq!(t("키읔이"), "키으기");
        assert_eq!(t("키읔을"), "키으글");
        assert_eq!(t("키읔에"), "키으게");
        assert_eq!(t("티읕이"), "티으시");
        assert_eq!(t("티읕을"), "티으슬");
        assert_eq!(t("티읕에"), "티으세");
        assert_eq!(t("피읖이"), "피으비");
        assert_eq!(t("피읖을"), "피으블");
        assert_eq!(t("피읖에"), "피으베");
        assert_eq!(t("히읗이"), "히으시");
        assert_eq!(t("히읗을"), "히으슬");
        assert_eq!(t("히읗에"), "히으세");
    }

    #[test]
    fn test_rule_17() {
		assert_eq!(t("곧이듣다"), "고지듣따");
		assert_eq!(t("굳이"), "구지");
		assert_eq!(t("미닫이"), "미다지");
		assert_eq!(t("땀받이"), "땀바지");
		assert_eq!(t("밭이"), "바치");
		assert_eq!(t("벼훑이"), "벼훌치");

        assert_eq!(t("굳히다"), "구치다");
        assert_eq!(t("닫히다"), "다치다");
        assert_eq!(t("묻히다"), "무치다");
    }

    #[test]
    fn test_rule_18() {
        assert_eq!(t("먹는"), "멍는");
        assert_eq!(t("국물"), "궁물");
        assert_eq!(t("깎는"), "깡는");
        assert_eq!(t("키읔만"), "키응만");
        assert_eq!(t("몫몫이"), "몽목씨");
        assert_eq!(t("긁는"), "긍는");
        assert_eq!(t("흙만"), "흥만");
        assert_eq!(t("닫는"), "단는");
        assert_eq!(t("짓는"), "진는");
        assert_eq!(t("옷맵시"), "온맵씨");
        assert_eq!(t("있는"), "인는");
        assert_eq!(t("맞는"), "만는");
        assert_eq!(t("젖멍울"), "전멍울");
        assert_eq!(t("쫓는"), "쫀는");
        assert_eq!(t("꽃망울"), "꼰망울");
        assert_eq!(t("붙는"), "분는");
        assert_eq!(t("놓는"), "논는");
        assert_eq!(t("잡는"), "잠는");
        assert_eq!(t("밥물"), "밤물");
        assert_eq!(t("앞마당"), "암마당");
        assert_eq!(t("밟는"), "밤는");
        assert_eq!(t("읊는"), "음는");
        assert_eq!(t("없는"), "엄는");
    }

    #[test]
    fn test_rule_19() {
        assert_eq!(t("담력"), "담녁");
        assert_eq!(t("침략"), "침냑");
        assert_eq!(t("강릉"), "강능");
        assert_eq!(t("항로"), "항노");
        assert_eq!(t("대통령"), "대통녕");

        assert_eq!(t("막론"), "망논");
        assert_eq!(t("석류"), "성뉴");
        assert_eq!(t("협력"), "혐녁");
        assert_eq!(t("법리"), "범니");
    }

    #[test]
    fn test_rule_20() {
        assert_eq!(t("난로"), "날로");
        assert_eq!(t("신라"), "실라");
        assert_eq!(t("천리"), "철리");
        assert_eq!(t("광한루"), "광할루");
        assert_eq!(t("대관령"), "대괄령");
        assert_eq!(t("칼날"), "칼랄");
        assert_eq!(t("물난리"), "물랄리");
        // assert_eq!(t("줄넘기"), "줄럼끼");
        // assert_eq!(t("할는지"), "할른지");
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

    #[test]
    fn test_rule_24() {
        assert_eq!(t("신고"), "신꼬");
        assert_eq!(t("껴안다"), "껴안따");
        assert_eq!(t("앉고"), "안꼬");
        assert_eq!(t("얹다"), "언따");
        assert_eq!(t("삼고"), "삼꼬");
        assert_eq!(t("더듬지"), "더듬찌");
        assert_eq!(t("닮고"), "담꼬");
        assert_eq!(t("젊지"), "점찌");
    }

    #[test]
    fn test_rule_25() {
        assert_eq!(t("넓게"), "널께");
        assert_eq!(t("핥다"), "할따");
        assert_eq!(t("훑소"), "훌쏘");
        assert_eq!(t("떫지"), "떨찌");
    }
}
