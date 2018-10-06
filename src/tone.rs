use std::fmt;

// 声调和音节:
//     https://zh.wikipedia.org/wiki/%E8%AA%BF%E5%80%BC
//     http://ccl.pku.edu.cn/doubtfire/Course/Modern%20Chinese/1_phonetics/Chapter_06_phonetics_tone%20syllable.pdf
// 
// "a", "ā", "á", "ǎ", "à",
// "e", "ē", "é", "ě", "è",
// "o", "ō", "ó", "ǒ", "ò",
// "i", "ī", "í", "ǐ", "ì",
// "u", "ū", "ú", "ǔ", "ù",
// "ü", "ǖ", "ǘ", "ǚ", "ǜ",
// "ń", "ň", "ǹ",
// "ḿ", "m̀",


pub const TONE_MARK_TABLE: [(&str, char, Tone); 35] = [
    ("a", 'a', Tone::Neutral), ("ā", 'a', Tone::First), ("á", 'a', Tone::Second), ("ǎ", 'a', Tone::Third), ("à", 'a', Tone::Fourth),
    ("e", 'e', Tone::Neutral), ("ē", 'e', Tone::First), ("é", 'e', Tone::Second), ("ě", 'e', Tone::Third), ("è", 'e', Tone::Fourth),
    ("o", 'o', Tone::Neutral), ("ō", 'o', Tone::First), ("ó", 'o', Tone::Second), ("ǒ", 'o', Tone::Third), ("ò", 'o', Tone::Fourth),
    ("i", 'i', Tone::Neutral), ("ī", 'i', Tone::First), ("í", 'i', Tone::Second), ("ǐ", 'i', Tone::Third), ("ì", 'i', Tone::Fourth),
    ("u", 'u', Tone::Neutral), ("ū", 'u', Tone::First), ("ú", 'u', Tone::Second), ("ǔ", 'u', Tone::Third), ("ù", 'u', Tone::Fourth),
    ("ü", 'ü', Tone::Neutral), ("ǖ", 'ü', Tone::First), ("ǘ", 'ü', Tone::Second), ("ǚ", 'ü', Tone::Third), ("ǜ", 'ü', Tone::Fourth),
    
    ("ń", 'n', Tone::Second), ("ň", 'n', Tone::Third), ("ǹ", 'n', Tone::Fourth),
    ("ḿ", 'm', Tone::Second), ("m̀", 'm', Tone::Fourth),
];

// 上标数字: ⁰¹²³⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾ⁿⁱ


/// 音调标记方式
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ToneFormat {
    /// 带音调符号的拼音字母 ( fan, fān )
    Symbol,
    /// 数字法 ( fan, fɑn⁵⁵, fan³⁵ )
    Digit,
    /// 声序法 ( fan, fan1 )
    Index,
}


/// 声调
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Tone {
    /// 第一声: 平调(阴平)
    First,
    /// 第二声: 升调(阳平)
    Second,
    /// 第三声: 降升语调(上升或折调)
    Third,
    /// 第四声: 降调(去声)
    Fourth,
    /// 第零声: 轻声调(不标符号)
    Neutral,
}

impl<'a> Into<u8> for &'a Tone {
    fn into(self) -> u8 {
        use self::Tone::*;

        match self {
            Neutral => 0u8,
            First => 1,
            Second => 2,
            Third => 3,
            Fourth => 4,
        }
    }
}
impl Into<u8> for Tone {
    fn into(self) -> u8 {
        (&self).into()
    }
}

impl Tone {
    pub fn name(&self) -> Option<&'static str> {
        use self::Tone::*;

        match *self {
            Neutral => None,
            First => Some("第一声"),
            Second => Some("第二声"),
            Third => Some("第三声"),
            Fourth => Some("第四声"),
        }
    }

    pub fn traditional_name(&self) -> Option<&'static str> {
        use self::Tone::*;

        match *self {
            Neutral => None,
            First => Some("阴平"),
            Second => Some("阳平"),
            Third => Some("上声"),
            Fourth => Some("去声"),
        }
    }

    pub fn descp(&self) -> Option<&'static str> {
        use self::Tone::*;

        match *self {
            Neutral => None,
            First => Some("高平调"),
            Second => Some("高升调"),
            Third => Some("降升调"),
            Fourth => Some("高降调"),
        }
    }

    pub fn mask(&self) -> Option<char> {
        use self::Tone::*;

        match *self {
            Neutral => None,
            First => Some('¯'),
            Second => Some('ˊ'),
            Third => Some('ˇ'),
            Fourth => Some('ˋ'),
        }
    }

    // 调值
    pub fn value(&self) -> Option<u8> {
        use self::Tone::*;

        match *self {
            Neutral => None,
            First => Some(55u8),
            Second => Some(35u8),
            Third => Some(214u8),
            Fourth => Some(51u8),
        }
    }
}

/// 发声字母以及声调
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ToneMark(char, Tone);

impl ToneMark {
    pub fn new(c: char, tone: Tone) -> Result<Self, ()> {
        let res=  TONE_MARK_TABLE.iter()
                    .filter(|(_s, cc, t)| &c == cc && &tone == t)
                    .map(|(_s, cc, t)| ToneMark(*cc, *t))
                    .collect::<Vec<ToneMark>>();
        
        if res.len() == 0 {
            Err(())
        } else if res.len() == 1 {
            Ok(res[0])
        } else {
            unreachable!();
        }
    }

    // 寻找音调字母
    pub fn find(s: &str) -> Vec<Self> {
        TONE_MARK_TABLE.iter()
            .filter(|(k, _c, _t)| s.contains(k))
            .map(|(_k, c, t)| ToneMark(*c, *t))
            .collect::<Vec<ToneMark>>()
    }
    
    // 替换 音调 字母 为普通字母
    pub fn replace_tone_marks(_s: &str) -> String {
        unimplemented!()
    }

    pub fn mark(&self) -> char {
        self.0
    }

    pub fn tone(&self) -> Tone {
        self.1
    }
}

impl fmt::Display for ToneMark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res=  TONE_MARK_TABLE.iter()
                    .filter(|(_s, c, t)| &self.0 == c && &self.1 == t)
                    .map(|(s, _c, _t)| s.to_string())
                    .collect::<Vec<String>>();
        
        assert_eq!(res.len(), 1);

        write!(f, "{}", res[0])
    }
}
