use std::fmt;
use std::cmp;


// 对《汉语拼音》方案当中的韵母表勘误:
//     韵母 `un` 实际上应该为 `uen`, 在书写的时候如果前面携带声母则写成 `un` 。
//     韵母 `ie` 和 `üe` 实际上应该为 `iê` 和 `üê` ，在书写的时候 如果 韵母 `ê` 不是单独存在的，则写成 `e`
// 
// 关于韵母 `-i（前）` 和 `-i（后）` ，这个在 《汉语拼音方案》当中被当成整体认读音节，所以不再区分他的韵母地位。
// 
// -i（前）这个韵母只和zcs有拼合关系，例如：紫，字，子
// -i（后）这个韵母只和zh ch sh r 有拼合关系，例如：之，指 ，支持，吃食，史诗


// ["a", "o", "e", "ê", "er", "ai", "ei", "ao", "ou", "an", "en", "ang", "eng", "ong"]
pub const RHYME_TABLE_COLUMN_A : [[char; 4]; 14] = [
    ['a', ' ', ' ', ' '], ['o', ' ', ' ', ' '], ['e', ' ', ' ', ' '],
    ['ê', ' ', ' ', ' '], ['e', 'r', ' ', ' '], ['a', 'i', ' ', ' '],
    ['e', 'i', ' ', ' '], ['a', 'o', ' ', ' '], ['o', 'u', ' ', ' '],
    ['a', 'n', ' ', ' '], ['e', 'n', ' ', ' '], ['a', 'n', 'g', ' '],
    ['e', 'n', 'g', ' '], ['o', 'n', 'g', ' '], 
];
// ["i", "ia", "ie", "iao", "iou", "ian", "in", "iang", "ing", "iong"]
pub const RHYME_TABLE_COLUMN_I : [[char; 4]; 10] = [
    ['i', ' ', ' ', ' '], ['i', 'a', ' ', ' '], ['i', 'e', ' ', ' '], 
    ['i', 'a', 'o', ' '], ['i', 'o', 'u', ' '], ['i', 'a', 'n', ' '], 
    ['i', 'n', ' ', ' '], ['i', 'a', 'n', 'g'], ['i', 'n', 'g', ' '], 
    ['i', 'o', 'n', 'g'], 
];
// ["u", "ua", "uo", "uai", "uei", "uan", "uen", "uang", "ueng"]
pub const RHYME_TABLE_COLUMN_U : [[char; 4]; 9] = [
    ['u', ' ', ' ', ' '], ['u', 'a', ' ', ' '], ['u', 'o', ' ', ' '], 
    ['u', 'a', 'i', ' '], ['u', 'e', 'i', ' '], ['u', 'a', 'n', ' '], 
    ['u', 'e', 'n', ' '], ['u', 'a', 'n', 'g'], ['u', 'e', 'n', 'g'], 
];
// ["ü", "üe", "üan", "ün"]
pub const RHYME_TABLE_COLUMN_YU: [[char; 4]; 4] = [
    ['ü', ' ', ' ', ' '], ['ü', 'e', ' ', ' '], ['ü', 'a', 'n', ' '], 
    ['ü', 'n', ' ', ' '], 
];

// NOTE: 为了视觉统一，该表并未排序，所以请不要使用 二分查找 之类的算法。
//       在《汉语拼音方案》当中 `un` 被当成了 韵母，实际上应该是 `uen` 。

// "a", "o", "e", "ê", "er", "ai", "ei", "ao", "ou", "an", "en", "ang", "eng", "ong",
// "i", "ia", "ie", "iao", "iou", "ian", "in", "iang", "ing", "iong",
// "u", "ua", "uo", "uai", "uei", "uan", "uen", "uang", "ueng",
// "ü", "üe", "üan", "ün",
pub const RHYME_TABLE: [[char; 4]; 37] = [
    ['a', ' ', ' ', ' '], ['o', ' ', ' ', ' '], ['e', ' ', ' ', ' '],
    ['ê', ' ', ' ', ' '], ['e', 'r', ' ', ' '], ['a', 'i', ' ', ' '],
    ['e', 'i', ' ', ' '], ['a', 'o', ' ', ' '], ['o', 'u', ' ', ' '],
    ['a', 'n', ' ', ' '], ['e', 'n', ' ', ' '], ['a', 'n', 'g', ' '],
    ['e', 'n', 'g', ' '], ['o', 'n', 'g', ' '], 
    ['i', ' ', ' ', ' '], ['i', 'a', ' ', ' '], ['i', 'e', ' ', ' '], 
    ['i', 'a', 'o', ' '], ['i', 'o', 'u', ' '], ['i', 'a', 'n', ' '], 
    ['i', 'n', ' ', ' '], ['i', 'a', 'n', 'g'], ['i', 'n', 'g', ' '], 
    ['i', 'o', 'n', 'g'], 
    ['u', ' ', ' ', ' '], ['u', 'a', ' ', ' '], ['u', 'o', ' ', ' '], 
    ['u', 'a', 'i', ' '], ['u', 'e', 'i', ' '], ['u', 'a', 'n', ' '], 
    ['u', 'e', 'n', ' '], ['u', 'a', 'n', 'g'], ['u', 'e', 'n', 'g'], 
    ['ü', ' ', ' ', ' '], ['ü', 'e', ' ', ' '], ['ü', 'a', 'n', ' '], 
    ['ü', 'n', ' ', ' '], 
];

// 单元音韵母表
// a, o, e, ê, er, i, ia, ie, u, ua, uo, ü, üe
pub const SIMPLE_VOWEL_RHYME_TABLE: [[char; 4]; 13] = [
    ['a', ' ', ' ', ' '], ['o', ' ', ' ', ' '], ['e', ' ', ' ', ' '],
    ['ê', ' ', ' ', ' '], ['e', 'r', ' ', ' '], ['i', ' ', ' ', ' '],
    ['i', 'a', ' ', ' '], ['i', 'e', ' ', ' '], ['u', ' ', ' ', ' '],
    ['u', 'a', ' ', ' '], ['u', 'o', ' ', ' '], ['ü', ' ', ' ', ' '],
    ['ü', 'e', ' ', ' '], 
];


// 复元音韵母表
// ai, ao, ei, iao, iou, ou, uai, uei
pub const COMPOUND_VOWEL_RHYME_TABLE: [[char; 4]; 8] = [
    ['a', 'i', ' ', ' '], ['a', 'o', ' ', ' '], ['e', 'i', ' ', ' '],
    ['i', 'a', 'o', ' '], ['i', 'o', 'u', ' '], ['o', 'u', ' ', ' '],
    ['u', 'a', 'i', ' '], ['u', 'e', 'i', ' '], 
];


/// 韵母
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Rhyme([char; 4]);

impl Rhyme {
    pub fn new(chars: [char; 4]) -> Result<Self, ()> {
        if RHYME_TABLE.contains(&chars) {
            Ok(Rhyme(chars))
        } else {
            Err(())
        }
    }

    // https://zh.wikipedia.org/wiki/%E6%B1%89%E8%AF%AD%E6%8B%BC%E9%9F%B3#%E5%A3%B0%E8%B0%83
    /// 声调标注规则
    pub fn tone_mark_rule(rhyme: &[char; 4]) -> Result<char, ()> {
        if rhyme.contains(&'a') {
            return Ok('a');
        }

        if rhyme.contains(&'o') && rhyme.contains(&'e') {
            // NOTE: 不能同时出现这两个字母
            return Err(());
        }

        if rhyme.contains(&'o') {
            return Ok('o');
        }
        if rhyme.contains(&'e') {
            return Ok('e');
        }

        let search = |c, rhyme: &[char]| -> Result<usize, ()> {
            for n in 0..rhyme.len() {
                if &rhyme[n] == c {
                    return Ok(n);
                }
            }
            Err(())
        };
        
        let pos_u2 = search(&'ü', rhyme);
        let pos_i = search(&'i', rhyme);
        let pos_u = search(&'u', rhyme);

        if pos_u2.is_ok() {
            if pos_i.is_ok() || pos_u.is_ok() {
                // `ü` 不可能和 `i` 或 `u` 同时出现
                return Err(());
            }

            return Ok('ü');
        }

        if pos_u.is_ok() && pos_i.is_ok() {
            // 如果 `i` 和 `u` 同时出现，则标在第二个韵母上
            let u_index = pos_u.unwrap();
            let i_index = pos_i.unwrap();
            let i = cmp::max(u_index, i_index);
            if i == u_index {
                return Ok('u');
            } else if i == i_index {
                return Ok('i');
            } else {
                unreachable!();
            }
        }

        if pos_i.is_ok() {
            return Ok('i');
        }

        if pos_u.is_ok() {
            return Ok('u');
        }

        // NOTE: 不合规范的韵母部分
        return Err(())
    }

    // 带声调的元音字母
    pub fn vowel(&self) -> char {
        match Rhyme::tone_mark_rule(&self.0) {
            Ok(c) => c,
            Err(_) => {
                println!("[DEBUG] 在对 `{}` 计算声调位置时，出现未预料的错误。", self);
                panic!("声调位置计算出现未预料的错误！");
            }
        }
    }
    
    // 判断是否为 单元音韵母
    pub fn is_simple(&self) -> bool {
        SIMPLE_VOWEL_RHYME_TABLE.contains(&self.0)
    }

    // 判断是否为 复元音韵母
    pub fn is_compound(&self) -> bool {
        COMPOUND_VOWEL_RHYME_TABLE.contains(&self.0)
    }

    // 判断是否带鼻音韵母
    pub fn is_nasal(&self) -> bool {
        // an, ian, uan, üan, en, uen, in, ün, ang, iang, uang, eng, ing, ueng, ong, iong
        let mut i = 3usize;

        while self.0[i] != ' ' {
            i -= 1;
        }

        let last_char = self.0[i];

        if last_char == 'n' || last_char == 'g' {
            true
        } else {
            false
        }
    }
}

impl fmt::Display for Rhyme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for elem in self.0.iter() {
            if elem == &' ' {
                break;
            }

            write!(f, "{}", elem);
        }

        Ok(())
    }
}

