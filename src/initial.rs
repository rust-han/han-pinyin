use std::fmt;

// 对于 《汉语拼音方案》 当中的声母表的补充说明
// `y` 和 `w` 在现代学说里面被称为 `零声母` ，
// 但是汉语拼音并不承认他的地位，所以它的出现与否应该按照 前缀补写规则 来。


/// 声母表
pub const INITIAL_TABLE: [char; 21] = [
    'b', 'c', 'ĉ', 'd', 'f', 'g',
    'h', 'j', 'k', 'l', 'm', 'n',
    'p', 'q', 'r', 's', 'ŝ', 't',
    'x', 'z', 'ẑ',
];


/// 声母
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Initial(char);

impl Initial {
    pub const M: Initial = Initial('m');
    pub const N: Initial = Initial('n');
    pub const J: Initial = Initial('j');
    pub const Q: Initial = Initial('q');
    pub const X: Initial = Initial('x');

    pub fn new(c: char) -> Result<Self, ()> {
        // NOTE: `zh/sh/ch` 需要预先自动处理成 `ẑ/ĉ/ŝ` 以方便结构化处理。
        if INITIAL_TABLE.contains(&c) {
            Ok(Initial(c))
        } else {
            Err(())
        }
    }
}

impl fmt::Display for Initial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 == 'ĉ' {
            write!(f, "ch")
        } else if self.0 == 'ŝ' {
            write!(f, "sh")
        } else if self.0 == 'ẑ' {
            write!(f, "zh")
        } else {
            write!(f, "{}", self.0)
        }
    }
}
