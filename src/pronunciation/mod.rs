mod rules;

use ::Word;
use self::rules::*;

pub struct Pronouncer {
    strict: bool,
}

impl Pronouncer {
    pub fn new(strict: bool) -> Pronouncer {
        Pronouncer { strict: strict }
    }

    pub fn pronounce(&self, word: &Word) -> Word {
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

            let (u, v) = rule_24(self, u, v);
            let (u, v) = rule_25(self, u, v);

            let u = if self.strict {
                let u = rule_05_1(u);
                let u = rule_05_3(u);
                u
            } else {
                u
            };

            let (u, v) = rule_12_1(u, v);

            let (u, v) = if self.strict {
                let (u, v) = rule_12_1_1(u, v);
                let (u, v) = rule_12_1_2(u, v);
                (u, v)
            } else {
                (u, v)
            };

            let (u, v) = rule_12_2(u, v);
            let (u, v) = rule_12_3(u, v);
            let (u, v) = rule_12_4(u, v);

            let u = rule_09(u, v);
            let u = rule_10(u, v);
            let u = rule_11(u, v);

            let (u, v) = rule_14(self, u, v);
            let (u, v) = rule_20(u, v);
            let (u, v) = rule_23(self, u, v);

            let (u, v) = rule_13(u, v);

            res.push(u.as_char());

            s = v;
            t = it.next().map(|t| *t);
        }

        Word::new(res)
    }

    // Transforms the given plain consonant into a tensed one.
    //
    // This is effectively no-op when `reflect_tenses` is false.
    pub fn reflect_tense(&self, j: char) -> char {
        if !self.strict{
            match j {
                'ㄱ' | 'ㄷ' | 'ㅂ' |  'ㅅ' |  'ㅈ' => j,
                _ => unreachable!(),
            }
        } else {
            match j {
                'ㄱ' => 'ㄲ',
                'ㄷ' => 'ㄸ',
                'ㅂ' => 'ㅃ',
                'ㅅ' => 'ㅆ',
                'ㅈ' => 'ㅉ',
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use ::Word;
    use super::Pronouncer;

    fn p(s: &str) -> String {
        let word = Word::new(s);
        let pronouncer = Pronouncer::new(true);
        pronouncer.pronounce(&word).to_string()
    }

    #[test]
    fn test_rule_05_1() {
        assert_eq!(p("가져"), "가저");
        assert_eq!(p("쪄"), "쩌");
        assert_eq!(p("다쳐"), "다처");
    }

    #[test]
    fn test_rule_09() {
        assert_eq!(p("닦다"), "닥따");
        assert_eq!(p("키읔"), "키윽");
        assert_eq!(p("키읔과"), "키윽꽈");
        assert_eq!(p("옷"), "옫");
        assert_eq!(p("웃다"), "욷따");
        assert_eq!(p("있다"), "읻따");
        assert_eq!(p("젖"), "젇");
        assert_eq!(p("빚다"), "빋따");
        assert_eq!(p("꽃"), "꼳");
        assert_eq!(p("쫓다"), "쫃따");
        assert_eq!(p("솥"), "솓");
        assert_eq!(p("뱉다"), "밷따");
        assert_eq!(p("앞"), "압");
        assert_eq!(p("덮다"), "덥따");
    }

    #[test]
    fn test_rule_10() {
        assert_eq!(p("넋"), "넉");
        assert_eq!(p("넋과"), "넉꽈");
        assert_eq!(p("앉다"), "안따");
        assert_eq!(p("여덟"), "여덜");
        assert_eq!(p("넓다"), "널따");
        assert_eq!(p("외곬"), "외골");
        assert_eq!(p("핥다"), "할따");
        assert_eq!(p("값"), "갑");
        assert_eq!(p("없다"), "업따");

        assert_eq!(p("밟다"), "밥따");
        assert_eq!(p("밟소"), "밥쏘");
        assert_eq!(p("밟지"), "밥찌");
        assert_eq!(p("밟는"), "밤는");
        assert_eq!(p("밟게"), "밥께");
        assert_eq!(p("밟고"), "밥꼬");
    }

    #[test]
    fn test_rule_11() {
        assert_eq!(p("닭"), "닥");
        assert_eq!(p("흙과"), "흑꽈");
        assert_eq!(p("맑다"), "막따");
        assert_eq!(p("늙지"), "늑찌");
        assert_eq!(p("삶"), "삼");
        assert_eq!(p("젊다"), "점따");
        assert_eq!(p("읊고"), "읍꼬");
        assert_eq!(p("읊다"), "읍따");
    }

    #[test]
    fn test_rule_12_1() {
        assert_eq!(p("놓고"), "노코");
        assert_eq!(p("좋던"), "조턴");
        assert_eq!(p("쌓지"), "싸치");
        assert_eq!(p("많고"), "만코");
        assert_eq!(p("않던"), "안턴");
        assert_eq!(p("닳지"), "달치");

   }

    #[test]
    fn test_rule_12_1_1() {
        assert_eq!(p("각하"), "가카");
        assert_eq!(p("먹히다"), "머키다");
        assert_eq!(p("밝히다"), "발키다");
        assert_eq!(p("맏형"), "마텽");
        assert_eq!(p("좁히다"), "조피다");
        assert_eq!(p("넓히다"), "널피다");
        assert_eq!(p("꽂히다"), "꼬치다");
        assert_eq!(p("앉히다"), "안치다");
    }

    #[test]
    fn test_rule_12_1_2() {
        assert_eq!(p("옷한벌"), "오탄벌");
        // assert_eq!(p("낮한때"), "나탄때");
        // assert_eq!(p("꽃한송이"), "꼬탄송이");
        assert_eq!(p("숱하다"), "수타다");
    }

    #[test]
    fn test_rule_12_2() {
        assert_eq!(p("닿소"), "다쏘");
        assert_eq!(p("많소"), "만쏘");
        assert_eq!(p("싫소"), "실쏘");
    }

    #[test]
    fn test_rule_12_3() {
        assert_eq!(p("놓는"), "논는");
        assert_eq!(p("쌓네"), "싼네");

        assert_eq!(p("않네"), "안네");
        assert_eq!(p("않는"), "안는");
        assert_eq!(p("뚫네"), "뚤레");
        assert_eq!(p("뚫는"), "뚤른");
    }

    #[test]
    fn test_rule_12_4() {
        assert_eq!(p("낳은"), "나은");
        assert_eq!(p("놓아"), "노아");
        assert_eq!(p("쌓이다"), "싸이다");
        assert_eq!(p("많아"), "마나");
        assert_eq!(p("않은"), "아는");
        assert_eq!(p("닳아"), "다라");
        assert_eq!(p("싫어도"), "시러도");
    }

    #[test]
    fn test_rule_13() {
        assert_eq!(p("깎아"), "까까");
        assert_eq!(p("옷이"), "오시");
        assert_eq!(p("있어"), "이써");
        assert_eq!(p("낮이"), "나지");
        assert_eq!(p("꽂아"), "꼬자");
        assert_eq!(p("꽃을"), "꼬츨");
        assert_eq!(p("쫓아"), "쪼차");
        assert_eq!(p("밭에"), "바테");
        assert_eq!(p("앞으로"), "아프로");
        assert_eq!(p("덮이다"), "더피다");
    }

    #[test]
    fn test_rule_14() {
        assert_eq!(p("넋이"), "넉씨");
        assert_eq!(p("앉아"), "안자");
        assert_eq!(p("닭을"), "달글");
        assert_eq!(p("젊어"), "절머");
        assert_eq!(p("곬이"), "골씨");
        assert_eq!(p("핥아"), "할타");
        assert_eq!(p("읊어"), "을퍼");
        assert_eq!(p("값을"), "갑쓸");
        assert_eq!(p("없어"), "업써");
    }

    #[test]
    fn test_rule_16() {
        assert_eq!(p("디귿이"), "디그시");
        assert_eq!(p("디귿을"), "디그슬");
        assert_eq!(p("디귿에"), "디그세");
        assert_eq!(p("지읒이"), "지으시");
        assert_eq!(p("지읒을"), "지으슬");
        assert_eq!(p("지읒에"), "지으세");
        assert_eq!(p("치읓이"), "치으시");
        assert_eq!(p("치읓을"), "치으슬");
        assert_eq!(p("치읓에"), "치으세");
        assert_eq!(p("키읔이"), "키으기");
        assert_eq!(p("키읔을"), "키으글");
        assert_eq!(p("키읔에"), "키으게");
        assert_eq!(p("티읕이"), "티으시");
        assert_eq!(p("티읕을"), "티으슬");
        assert_eq!(p("티읕에"), "티으세");
        assert_eq!(p("피읖이"), "피으비");
        assert_eq!(p("피읖을"), "피으블");
        assert_eq!(p("피읖에"), "피으베");
        assert_eq!(p("히읗이"), "히으시");
        assert_eq!(p("히읗을"), "히으슬");
        assert_eq!(p("히읗에"), "히으세");
    }

    #[test]
    fn test_rule_17() {
        assert_eq!(p("곧이듣다"), "고지듣따");
        assert_eq!(p("굳이"), "구지");
        assert_eq!(p("미닫이"), "미다지");
        assert_eq!(p("땀받이"), "땀바지");
        assert_eq!(p("밭이"), "바치");
        assert_eq!(p("벼훑이"), "벼훌치");

        assert_eq!(p("굳히다"), "구치다");
        assert_eq!(p("닫히다"), "다치다");
        assert_eq!(p("묻히다"), "무치다");
    }

    #[test]
    fn test_rule_18() {
        assert_eq!(p("먹는"), "멍는");
        assert_eq!(p("국물"), "궁물");
        assert_eq!(p("깎는"), "깡는");
        assert_eq!(p("키읔만"), "키응만");
        assert_eq!(p("몫몫이"), "몽목씨");
        assert_eq!(p("긁는"), "긍는");
        assert_eq!(p("흙만"), "흥만");
        assert_eq!(p("닫는"), "단는");
        assert_eq!(p("짓는"), "진는");
        assert_eq!(p("옷맵시"), "온맵씨");
        assert_eq!(p("있는"), "인는");
        assert_eq!(p("맞는"), "만는");
        assert_eq!(p("젖멍울"), "전멍울");
        assert_eq!(p("쫓는"), "쫀는");
        assert_eq!(p("꽃망울"), "꼰망울");
        assert_eq!(p("붙는"), "분는");
        assert_eq!(p("놓는"), "논는");
        assert_eq!(p("잡는"), "잠는");
        assert_eq!(p("밥물"), "밤물");
        assert_eq!(p("앞마당"), "암마당");
        assert_eq!(p("밟는"), "밤는");
        assert_eq!(p("읊는"), "음는");
        assert_eq!(p("없는"), "엄는");
    }

    #[test]
    fn test_rule_19() {
        assert_eq!(p("담력"), "담녁");
        assert_eq!(p("침략"), "침냑");
        assert_eq!(p("강릉"), "강능");
        assert_eq!(p("항로"), "항노");
        assert_eq!(p("대통령"), "대통녕");

        assert_eq!(p("막론"), "망논");
        assert_eq!(p("석류"), "성뉴");
        assert_eq!(p("협력"), "혐녁");
        assert_eq!(p("법리"), "범니");
    }

    #[test]
    fn test_rule_20() {
        assert_eq!(p("난로"), "날로");
        assert_eq!(p("신라"), "실라");
        assert_eq!(p("천리"), "철리");
        assert_eq!(p("광한루"), "광할루");
        assert_eq!(p("대관령"), "대괄령");
        assert_eq!(p("칼날"), "칼랄");
        assert_eq!(p("물난리"), "물랄리");
        assert_eq!(p("줄넘기"), "줄럼끼");
        // assert_eq!(p("할는지"), "할른지");
    }

    #[test]
    fn test_rule_23() {
        assert_eq!(p("국밥"), "국빱");
        assert_eq!(p("깎다"), "깍따");
        assert_eq!(p("넋받이"), "넉빠지");
        assert_eq!(p("삯돈"), "삭똔");
        assert_eq!(p("닭장"), "닥짱");
        assert_eq!(p("칡범"), "칙뻠");
        assert_eq!(p("뻗대다"), "뻗때다");
        assert_eq!(p("옷고름"), "옫꼬름");
        assert_eq!(p("있던"), "읻떤");
        assert_eq!(p("꽂고"), "꼳꼬");
        assert_eq!(p("꽃다발"), "꼳따발");
        assert_eq!(p("낯설다"), "낟썰다");
        assert_eq!(p("밭갈이"), "받까리");
        assert_eq!(p("솥전"), "솓쩐");
        assert_eq!(p("곱돌"), "곱똘");
        assert_eq!(p("덮개"), "덥깨");
        assert_eq!(p("옆집"), "엽찝");
        // assert_eq!(p("넓죽하다"), "넙쭈카다");
        assert_eq!(p("읊조리다"), "읍쪼리다");
        assert_eq!(p("값지다"), "갑찌다");
    }

    #[test]
    fn test_rule_24() {
        assert_eq!(p("신고"), "신꼬");
        assert_eq!(p("껴안다"), "껴안따");
        assert_eq!(p("앉고"), "안꼬");
        assert_eq!(p("얹다"), "언따");
        assert_eq!(p("삼고"), "삼꼬");
        assert_eq!(p("더듬지"), "더듬찌");
        assert_eq!(p("닮고"), "담꼬");
        assert_eq!(p("젊지"), "점찌");
    }

    #[test]
    fn test_rule_25() {
        assert_eq!(p("넓게"), "널께");
        assert_eq!(p("핥다"), "할따");
        assert_eq!(p("훑소"), "훌쏘");
        assert_eq!(p("떫지"), "떨찌");
    }
}
