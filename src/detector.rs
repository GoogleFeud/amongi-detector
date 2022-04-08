
use crate::analyzer::{Analyzer, Pixel};

pub trait Detector: Sync + Send {
    fn on_pixel(&self, analyzer: &Analyzer, pixel: &Pixel) -> Option<Vec<Pixel>>;
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
    ($analyzer: expr, $pixel: expr, $res: expr, $proximity: expr, ($(($($direction: ident $amount: expr)=>+, $proxy_minus: expr)),+)) => {
        let mut proximity = $proximity;
        $(
            let upper = $pixel.$($direction(&$analyzer.data, $amount)?).+;
            if $pixel.2 == upper.2 {
                $res.push(upper);
            } else {
                proximity -= $proxy_minus;
                if proximity < 1 {
                    return None;
                }
            }
        )+
    };
}

pub(crate) use cmp_close_pixel;