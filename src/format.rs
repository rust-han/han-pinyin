use tone::ToneFormat;
use syllable::{ PrimitiveSyllable, NormalSyllable, RhymeSyllable, NasalSyllable };


pub trait Show {
    fn show(&self, tone_format: ToneFormat) -> Result<(), ()>;
}


impl Show for PrimitiveSyllable {
    fn show(&self, _tone_format: ToneFormat) -> Result<(), ()> {
        unimplemented!()
    }
}

impl Show for NormalSyllable {
    fn show(&self, _tone_format: ToneFormat) -> Result<(), ()> {
        unimplemented!()
    }
}

impl Show for RhymeSyllable {
    fn show(&self, _tone_format: ToneFormat) -> Result<(), ()> {
        unimplemented!()
    }
}

impl Show for NasalSyllable {
    fn show(&self, _tone_format: ToneFormat) -> Result<(), ()> {
        unimplemented!()
    }
}
