#![feature(pattern, try_from)]


mod tone;
mod initial;
mod rhyme;
mod syllable;
mod error;
mod format;

pub use tone::{ Tone, ToneMark, ToneFormat };
pub use initial::Initial;
pub use rhyme::Rhyme;
pub use syllable::{
    Syllable, SyllableKind,
    PrimitiveSyllable, NormalSyllable, RhymeSyllable, NasalSyllable,
    from_str,
};
pub use error::Error;

use std::str::Split;
use std::str::pattern::Pattern;


pub const SYLLABLE_DIVIDING_MARK: char = '\'';


#[doc(hidden)]
pub struct PinYinIter<'a, P: Pattern<'a>> {
    inner: Split<'a, P>,
}

impl<'a, P> Iterator for  PinYinIter<'a, P> where P: Pattern<'a> {
    type Item = Box<Syllable>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|s| -> Box<Syllable> {
            syllable::from_str(s).expect("数据库记录有误！")
        })
    }
}


pub trait PinYin<'a> {
    type Item;
    type Error;

    fn pinyin(&self) -> Result<Self::Item, Self::Error>;
}

impl<'a> PinYin<'a> for char {
    type Item = PinYinIter<'a, char>;
    type Error = ();

    fn pinyin(&self) -> Result<Self::Item, Self::Error> {
        // PINYIN_MAP.binary_search_by_key(self, |&(k, _)| k)
        //     .map(|index| PinYinIter { inner: PINYIN_MAP[index].1.split(',') })
        //     .map_err(|_| () )
        unimplemented!()
    }
}
