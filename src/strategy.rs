use crate::types::{Candle, CandleLine, Signal};
use std::time::{Duration, Instant};

pub fn exs(candles: &Vec<Candle>) -> Signal {
    //let t = Instant::now();
    let cline = CandleLine::new_from_vec(candles.clone());
    let mut ha_cline = cline.heikinashi();

    //println!("Calculating {} bars of heikenashi took: {} microseconds",&candles.len(), t.elapsed().as_micros());
    println!("{:#?}\n------", ha_cline.get_range(ha_cline.len(),ha_cline.len()-4));

    Signal::Long
}

impl CandleLine {
    pub fn heikinashi(&self) -> CandleLine {
        // Method generating Heikin Ashi candlesticks, best use with buffer of at least 10
        // candles in past to decrease "synthetic" first candle
        let data = self.all();
        let mut ha = CandleLine::new();
        // Initial candle
        ha.push(Candle::new(
            data[0].timestamp(),
            (data[0].open() + data[0].close()) / 2.0,
            data[0].high(),
            data[0].low(),
            (data[0].open() + data[0].high() + data[0].low() + data[0].close()) / 4.0,
            data[0].volume(),
        ));

        // Add all remaining candles
        for i in 1..data.len() {
            let kline = Candle::new(
                data[i].timestamp(),
                (ha.get(i - 1).open() + ha.get(i - 1).close()) / 2.0, // This needs to be changed
                data[i].high(),
                data[i].low(),
                (data[i].open() + data[i].high() + data[i].low() + data[i].close()) / 4.0,
                data[i].volume(),
            );
            ha.push(kline);
        }
        ha
    }
}
