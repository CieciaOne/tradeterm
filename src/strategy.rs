use crate::types::{Candle,Signal};
pub fn exs(candles: &Vec<Candle>) -> Signal {
    Signal::Long
}
