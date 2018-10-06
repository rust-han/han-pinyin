extern crate pinyin;

use pinyin::{ PinYin, NormalSyllable };


pub fn main() {
    // let word: char = '他';

    // println!("查询汉字: {:?}\n", word);

    // for syllable in word.pinyin().unwrap() {
    //     println!("Syllable: {:?}", syllable);
    //     println!("Vowel: {:?}", syllable.vowel());
    //     println!("Tone: {:?}", syllable.tone());
    //     println!("ToneMark: {:?}", syllable.tone_mark());
    //     println!("");
    // }

    println!("{:?}", "ju".parse::<NormalSyllable>() );
    println!("{}", "ju".parse::<NormalSyllable>().unwrap() );
}
