# hangeul

**hanguel** is a Korean romanization library. It implements transcription rules
based on standard pronunciation specified by the [National Institute of Korean
Language](http://www.korean.go.kr/front_eng/main.do).

## Usage

### lib

```rust
use hangeul::romanize;

assert_eq!(romanize("우주소녀"), "ujusonyeo");
assert_eq!(romanize("러블리즈"), "reobeullijeu");
assert_eq!(romanize("에이핑크"), "eipingkeu");
```

### bin

```
$ hangeul <input>
```

## Definitions

A **syllable** is a precomposed cluster of positional **jamos**, or letters:
an initial consonant (**choseong**), a medial vowel (**jungseong**), and an
optional final consonant (**jongseong**).

**Romanization** is the conversion from one script to Roman, or Latin, script.
Two common methods are transliteration and transcription. **Transliteration**
maps characters from the source script to the target. **Transcription**
typically uses pronunciation rules from the source language to approximate how
it would sound in the target language.

## Limitations

  * Compound words are treated as single words, e.g.,
    "마지막처럼" => "majimakcheoreom".

  * Proper nouns with administrative units are neither capitalized nor
    hypenated, e.g., "제주도" => "jejudo" (expected: "Jeju-do").

  * Names are neither capitalized nor spaced, e.g., "김세정" => "gimsejeong"
    (expected: "Gim Se Jeong"). Note that names still commonly use
    [McCune-Reischauer] romanization and other de facto rules over Revised
    Romanization (see also "[Discussion of Surname Romanization]"). The
    previous example would more likely be "Kim Se Jeong" or "Kim Se Jung".

[McCune-Reischauer]: https://en.wikipedia.org/wiki/McCune%E2%80%93Reischauer
[Discussion of Surname Romanization]: http://korean.go.kr/front/etcData/etcDataView.do?etc_seq=179&mn_id=46

## Resources

  * [Romanization of Korean](http://www.korean.go.kr/front_eng/roman/roman_01.do) (English)
  * [Standard Language Specification: Part 2. Standard Pronunciation](https://www.korean.go.kr/front/page/pageView.do?page_id=P000097&mn_id=95) (Korean)
