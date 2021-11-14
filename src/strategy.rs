use crate::types::{Candle, CandleLine, Signal};
//use std::time::{Duration, Instant};

pub fn exs(candles: &Vec<Candle>) -> Signal {
    //let t = Instant::now();
    let cline = CandleLine::new_from_vec(candles.clone());
    let mut ha_cline = cline.heikinashi();

    //println!("Calculating {} bars of heikenashi took: {} microseconds",&candles.len(), t.elapsed().as_micros());
//    println!(
//        "{:#?}\n------",
//        ha_cline.get_range(ha_cline.len(), ha_cline.len() - 4)
//    );

    if cline.last().close() > ha_cline.last().close(){
        Signal::Long
    }
    else if cline.last().close() < ha_cline.get(ha_cline.len().wrapping_sub(1)).low(){
        Signal::Short
    }
    else{
        Signal::Sleep
    }
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

pub fn cross_over(a: Vec<f64>, b: Vec<f64>) -> bool {
    if (a[0] <= b[0]) & (a[1] > b[1]) {
        true
    } else {
        false
    }
}

pub fn moving_average(data: Vec<f64>, window: usize) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();

    for i in 0..data.len() {
        if i < window - 1 {
            result.push(data[i]);
        } else {
            let mut temp = result[i + 1 - window..i].to_vec();
            temp.push(data[i]);
            result.push(avg(&temp));
        }
    }
    result
}
pub fn avg(data: &Vec<f64>) -> f64 {
    data.clone().iter().sum::<f64>() / data.len() as f64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ma_test() {
        let base = vec![
            10.0,
            20.0,
            11.833333333333334,
            20.644444444444446,
            44.159259259259265,
        ];
        let calculated = moving_average(vec![10.0, 20.0, 5.5, 30.1, 100.0], 3);

        assert_eq!(base, calculated);
    }

    #[test]
    fn mean_test() {
        let data: Vec<f64> = vec![10.0, 0.0, 2.0, 5.0];
        assert_eq!(avg(&data), 4.25);
    }
}
