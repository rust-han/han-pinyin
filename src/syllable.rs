use tone::{Tone, ToneMark};
use initial::Initial;
use rhyme::{ Rhyme, RHYME_TABLE_COLUMN_I, RHYME_TABLE_COLUMN_U, RHYME_TABLE_COLUMN_YU };

use std::fmt;
use std::cmp;
use std::str::FromStr;


// zhi, chi, shi, ri, zi, ci, si, yi, wu, yu, ye, yue, yuan, yin, yun, ying
//   i,   i,   i,  i,  i,  i,  i,  i,  u,  u,  e,   e,    a,   i,   u,    i
// 整体认读音节表
pub const PRIMITIVE_SYLLABLE_TABLE: [([char; 4], char); 16] = [
    (['z', 'h', 'i', ' '], 'i'), (['c', 'h', 'i', ' '], 'i'), (['s', 'h', 'i', ' '], 'i'), 
    (['r', 'i', ' ', ' '], 'i'), (['z', 'i', ' ', ' '], 'i'), (['c', 'i', ' ', ' '], 'i'),
    (['s', 'i', ' ', ' '], 'i'), (['y', 'i', ' ', ' '], 'i'), (['w', 'u', ' ', ' '], 'u'),
    (['y', 'u', ' ', ' '], 'u'), (['y', 'e', ' ', ' '], 'e'), (['y', 'u', 'e', ' '], 'e'),
    (['y', 'u', 'a', 'n'], 'a'), (['y', 'i', 'n', ' '], 'i'), (['y', 'u', 'n', ' '], 'u'),
    (['y', 'i', 'n', 'g'], 'i'), 
];


/// 音节类型
#[derive(Debug, Copy, Clone)]
pub enum SyllableKind {
    /// 整体认读音节
    Primitive,
    /// 常规音节 ( 或 两拼音节 + 三拼音节 )
    Normal,
    /// 自成音节
    Rhyme,
    /// 鼻音音节 (注: 这个并不在 `汉语拼音` 规范当中，属于扩展性质)
    Nasal,
}

pub trait Syllable: fmt::Display + fmt::Debug {
    fn kind(&self) -> SyllableKind;
    fn initial(&self) -> Option<Initial>;
    fn finals(&self) -> Option<Rhyme>;
    fn vowel(&self) -> char;
    fn tone(&self) -> Tone;
    fn tone_mark(&self) -> ToneMark {
        ToneMark::new(self.vowel(), self.tone())
            .expect("错误的音节元音音调！")
    }
}


// zhi chi shi ri zi ci si yi wu yu ye yue yuan yin yun ying
/// 整体认读音节
#[derive(Debug)]
pub struct PrimitiveSyllable {
    primitive: [char; 4],
    vowel: char,
    tone: Tone,
}

impl PrimitiveSyllable {
    pub fn new(chars: [char; 4], tone: Tone) -> Result<Self, ()> {
        
        for (primitive, vowel) in PRIMITIVE_SYLLABLE_TABLE.iter() {
            if primitive == &chars {
                return Ok(PrimitiveSyllable{ primitive: chars, vowel: *vowel, tone });
            }
        }

        return Err(());
    }
}

impl Syllable for PrimitiveSyllable {
    fn kind(&self) -> SyllableKind {
        SyllableKind::Primitive
    }

    fn initial(&self) -> Option<Initial> {
        None
    }
    
    fn finals(&self) -> Option<Rhyme> {
        None
    }

    fn vowel(&self) -> char {
        self.vowel
    }

    fn tone(&self) -> Tone {
        self.tone
    }
}

impl fmt::Display for PrimitiveSyllable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tone_mark = self.tone_mark();

        let s = self.primitive.iter().collect::<String>()
            .replace(self.vowel, tone_mark.to_string().as_ref())
            .replace(' ', "");

        write!(f, "{}", s)
    }
}


/// 常规音节
#[derive(Debug)]
pub struct NormalSyllable {
    initial: Initial,
    rhyme: Rhyme,
    tone: Tone,
}

impl NormalSyllable {
    pub fn new(initial: Initial, rhyme: Rhyme, tone: Tone) -> Result<Self, ()> {
        Ok(NormalSyllable {
            initial,
            rhyme,
            tone,
        })
    }
}

impl Syllable for NormalSyllable {
    fn kind(&self) -> SyllableKind {
        SyllableKind::Primitive
    }

    fn initial(&self) -> Option<Initial> {
        Some(self.initial)
    }
    
    fn finals(&self) -> Option<Rhyme> {
        Some(self.rhyme)
    }

    fn vowel(&self) -> char {
        self.rhyme.vowel()
    }

    fn tone(&self) -> Tone {
        self.tone
    }
}

impl fmt::Display for NormalSyllable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut vowel = self.vowel();
        let mut tone_mark = self.tone_mark();

        // let u2a = ["ü", "üe", "üan", "ün"];
        // let u2b = ["yu", "yue", "yuan", "yun"];

        // 替换规则: iou、uei、uen 前面加声母的时候，写成 iu、ui、un，例如 niu（牛）、gui（归）、lun（论）。
        let mut s = format!("{}", self.rhyme)
                        .replace("iou", "iu")
                        .replace("uei", "ui")
                        .replace("uen", "un");

        if self.initial == Initial::J 
            || self.initial == Initial::Q 
            || self.initial == Initial::X {
            // ü 行的韵母跟声母 j，q，x 拼的时候，写成 ju（居），qu（取），xu（虚），jue（觉），que（缺），xue（学），ü 上两点也省略；
            // 但是跟声母 l，n 拼的时候，仍然写成 lü（吕），lüe（略），nü（女），nüe（虐）。
            // ['ü', ' ', ' ', ' '], ['ü', 'e', ' ', ' ']
            if &s == "ü" || &s == "üe" {
                // 执行替换规则
                vowel = 'u';
                tone_mark = ToneMark::new(vowel, self.tone).unwrap();
                s = s.replace('ü', "u");
            }
        }

        s = s.replace(vowel, tone_mark.to_string().as_ref())
                    .replace(' ', "");
        write!(f, "{}{}", self.initial, s)
    }
}


/// 自成音节（不包含声母，只有韵母部分）
#[derive(Debug)]
pub struct RhymeSyllable {
    rhyme: Rhyme,
    tone: Tone,
}

impl RhymeSyllable {
    pub fn new(rhyme: Rhyme, tone: Tone) -> Result<Self, ()> {
        Ok(RhymeSyllable {
            rhyme,
            tone,
        })
    }
}

impl Syllable for RhymeSyllable {
    fn kind(&self) -> SyllableKind {
        SyllableKind::Rhyme
    }

    fn initial(&self) -> Option<Initial> {
        None
    }
    
    fn finals(&self) -> Option<Rhyme> {
        Some(self.rhyme)
    }

    fn vowel(&self) -> char {
        self.rhyme.vowel()
    }

    fn tone(&self) -> Tone {
        self.tone
    }
}

impl fmt::Display for RhymeSyllable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vowel = self.vowel();
        let tone_mark = self.tone_mark();

        let mut s = format!("{}", self.rhyme)
                    .replace(vowel, tone_mark.to_string().as_ref())
                    .replace(' ', "");
        
        // 执行拼音方案当中的补写规则 ( i => yi, ia => ya, u => wu, ... )
        let ia = ["i", "ia", "ie", "iao", "iou", "ian", "in", "iang", "ing", "iong"];
        let ib = ["yi", "ya", "ye", "yao", "you", "yan", "yin", "yang", "ying", "yong"];

        let ua = ["u", "ua", "uo", "uai", "uei", "uan", "uen", "uang", "ueng"];
        let ub = ["wu", "wa", "wo", "wai", "wei", "wan", "wen", "wang", "weng"];
        
        let u2a = ["ü", "üe", "üan", "ün"];
        let u2b = ["yu", "yue", "yuan", "yun"];

        // BUG: 音调需要延迟处理
        for (a, b) in ia.iter().zip(ib.iter()) {
            if a == &s {
                s = s.replace(a, b);
            }
        }

        for (a, b) in ua.iter().zip(ub.iter()) {
            if a == &s {
                s = s.replace(a, b);
            }
        }

        for (a, b) in u2a.iter().zip(u2b.iter()) {
            if a == &s {
                s = s.replace(a, b);
            }
        }

        write!(f, "{}", s)
    }
}


/// 鼻音音节 （不属于汉语拼音规范）
#[derive(Debug)]
pub struct NasalSyllable {
    initial: Initial,
    tone: Tone,
}

impl NasalSyllable {
    pub fn new(initial: Initial, tone: Tone) -> Result<Self, ()> {
        //             'ń' | 'ň' | 'ǹ'
        //             'ḿ' |       "m̀"
        if initial == Initial::M {
            if tone == Tone::Second || tone == Tone::Fourth {
                Ok(NasalSyllable {
                    initial,
                    tone,
                })
            } else {
                Err(())
            }
        } else if initial == Initial::N {
            if tone == Tone::Second || tone == Tone::Third 
                || tone == Tone::Fourth {
                Ok(NasalSyllable {
                    initial,
                    tone,
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

impl Syllable for NasalSyllable {
    fn kind(&self) -> SyllableKind {
        SyllableKind::Nasal
    }

    fn initial(&self) -> Option<Initial> {
        Some(self.initial)
    }
    
    fn finals(&self) -> Option<Rhyme> {
        None
    }

    fn vowel(&self) -> char {
        if self.initial ==  Initial::M {
            'm'
        } else if self.initial == Initial::N {
            'n'
        } else {
            unreachable!()
        }
    }

    fn tone(&self) -> Tone {
        self.tone
    }
}

impl fmt::Display for NasalSyllable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.tone_mark())
    }
}



impl FromStr for PrimitiveSyllable {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tone_marks = ToneMark::find(&s).iter()
                            .cloned()
                            .filter(|tone_mark| tone_mark.tone() != Tone::Neutral)
                            .collect::<Vec<ToneMark>>();
        
        if tone_marks.len() > 1 {
            return Err(());
        }

        let mut tone: Tone = Tone::Neutral;
        
        let chars = if tone_marks.len() == 1 {
            let tone_mark = tone_marks[0];
            let mark = tone_mark.mark();
            tone = tone_mark.tone();
            s.replace(format!("{}", tone_mark).as_str(), mark.to_string().as_ref())
                .chars()
                .collect::<Vec<char>>()
        } else {
            s.to_string().chars().collect::<Vec<char>>()
        };

        let mut query: [char; 4] = [' '; 4];
        let n = cmp::min(query.len(), chars.len());

        for i in 0..n {
            query[i] = chars[i];
        }

        PrimitiveSyllable::new(query, tone)
    }
}

impl FromStr for NormalSyllable {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(());
        }

        let tone_marks = ToneMark::find(&s).iter()
                            .cloned()
                            .filter(|tone_mark| tone_mark.tone() != Tone::Neutral)
                            .collect::<Vec<ToneMark>>();
        
        if tone_marks.len() > 1 {
            return Err(());
        }

        let mut tone: Tone = Tone::Neutral;
        
        let text = if tone_marks.len() == 1 {
            let tone_mark = tone_marks[0];
            let mark = tone_mark.mark();
            tone = tone_mark.tone();
            s.replace(format!("{}", tone_mark).as_str(), mark.to_string().as_ref())
                .replace("zh", "ẑ")
                .replace("ch", "ĉ")
                .replace("sh", "ŝ")
        } else {
            s.replace("zh", "ẑ")
                .replace("ch", "ĉ")
                .replace("sh", "ŝ")
        };

        let chars = text.chars().collect::<Vec<char>>();
        let first_char = chars[0];

        match Initial::new(first_char) {
            Ok(initial) => {
                let mut finals = chars[1..].iter().collect::<String>();

                // NOTE: 还原规则
                if &finals == "iu" {
                    finals = "iou".to_string();
                }
                if &finals == "ui" {
                    finals = "uei".to_string();
                }
                if &finals == "un" {
                    finals = "uen".to_string();
                }

                if initial == Initial::J || initial == Initial::Q || initial == Initial::X {
                    if &finals == "u" || &finals == "ue" {
                        finals = finals.replace('u', "ü");
                    }
                }

                match RhymeSyllable::from_str(&finals) {
                    Ok(rhyme_syllable) => {
                        NormalSyllable::new(initial, rhyme_syllable.rhyme, tone)
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }
}

impl FromStr for RhymeSyllable {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 1 {
            return Err(());
        }

        let tone_marks = ToneMark::find(&s).iter()
                            .cloned()
                            .filter(|tone_mark| tone_mark.tone() != Tone::Neutral)
                            .collect::<Vec<ToneMark>>();
        
        if tone_marks.len() > 1 {
            return Err(());
        }

        let mut tone: Tone = Tone::Neutral;
        
        let mut text = if tone_marks.len() == 1 {
            let tone_mark = tone_marks[0];
            let mark = tone_mark.mark();
            tone = tone_mark.tone();
            s.replace(format!("{}", tone_mark).as_str(), mark.to_string().as_ref())
                .replace('ŋ', "ng")
        } else {
            s.replace('ŋ', "ng")
        };

        // `i` 列规则
        let ia = ["yi", "ya", "ye", "yao", "you", "yan", "yin", "yang", "ying", "yong"];
        for (a, b) in ia.iter().zip(RHYME_TABLE_COLUMN_I.iter()) {
            if a == &text {
                text = text.replace(a, b.iter().collect::<String>().replace(' ', "").as_str() );
            }
        }

        // `u` 列规则
        let ua = ["wu", "wa", "wo", "wai", "wei", "wan", "wen", "wang", "weng"];
        for (a, b) in ua.iter().zip(RHYME_TABLE_COLUMN_U.iter()) {
            if a == &text {
                text = text.replace(a, b.iter().collect::<String>().replace(' ', "").as_str() );
            }
        }

        // `ü` 列规则
        let u2a = ["yu", "yue", "yuan", "yun"];
        for (a, b) in u2a.iter().zip(RHYME_TABLE_COLUMN_YU.iter()) {
            if a == &text {
                text = text.replace(a, b.iter().collect::<String>().replace(' ', "").as_str() );
            }
        }

        let chars = text.chars().collect::<Vec<char>>();
        let mut query: [char; 4] = [' '; 4];

        let n = cmp::min(query.len(), chars.len());

        for i in 0..n {
            query[i] = chars[i];
        }

        match Rhyme::new(query) {
            Ok(rhyme) => RhymeSyllable::new(rhyme, tone),
            Err(e) => Err(e),
        }
    }
}


impl FromStr for NasalSyllable {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 'ń' | 'ň' | 'ǹ'
        // 'ḿ' |       "m̀"
        match s {
            "ń" => NasalSyllable::new(Initial::N, Tone::Second),
            "ň" => NasalSyllable::new(Initial::N, Tone::Third),
            "ǹ" => NasalSyllable::new(Initial::N, Tone::Fourth),
            "m̀" => NasalSyllable::new(Initial::M, Tone::Fourth),
            "ḿ" | "" => NasalSyllable::new(Initial::M, Tone::Second),
            _ => Err(()),
        }
    }
}


pub fn from_str(s: &str) -> Result<Box<Syllable>, ()> {
    if let Ok(v) = s.parse::<PrimitiveSyllable>() {
        Ok(Box::new(v))
    } else if let Ok(v) = s.parse::<NormalSyllable>() {
        Ok(Box::new(v))
    } else if let Ok(v) = s.parse::<RhymeSyllable>() {
        Ok(Box::new(v))
    } else if let Ok(v) = s.parse::<NasalSyllable>() {
        Ok(Box::new(v))
    } else {
        Err(())
    }
}
