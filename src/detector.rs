
use crate::analyzer::{Analyzer, Pixel};

pub trait Detector {
    fn on_pixel(&mut self, analyzer: &Analyzer, pixel: &Pixel) -> Option<Vec<Pixel>>;
}

macro_rules! cmp_pixel {
    ($analyzer: expr, $pixel: expr, $res: expr, ($(($($direction: ident $amount: expr)=>+)),+)) => {
        $(
            let upper = $pixel.$($direction(&$analyzer.data, $amount)?).+;
            if $pixel.2 == upper.2 {
                $res.push(upper);
            } else {
                return None;
            }
        )+
    };
}

pub(crate) use cmp_pixel;

macro_rules! cmp_close_pixel {
    ($analyzer: expr, $pixel: expr, $res: expr, $threshold: expr, $cmp: tt, ($(($($direction: ident $amount: expr)=>+, $proxy_minus: expr)),+)) => {
        let mut threshold = $threshold;
        $(
            let upper = $pixel.$($direction(&$analyzer.data, $amount)?).+;
            if $pixel.2 $cmp upper.2 {
                threshold -= $proxy_minus;
                if threshold < 1 {
                    return None;
                }
            }
        )+
    };
}

pub(crate) use cmp_close_pixel;