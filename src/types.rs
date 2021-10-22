use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    name: String,
    description: String,
    ticker: String,
    timeframe: String,
    window: usize,
    strategy: String,
    socket_url: String,
    api_url: String,
}
impl Config {
    pub fn new(
        name: String,
        description: String,
        ticker: String,
        timeframe: String,
        window: usize,
        strategy: String,
        socket_url: String,
        api_url: String,
    ) -> Config {
        Config {
            name: name.to_lowercase(),
            description,
            ticker,
            timeframe,
            window,
            strategy: strategy.to_lowercase(),
            socket_url,
            api_url,
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
    pub fn get_ticker(&self) -> String {
        self.ticker.clone()
    }
    pub fn get_timeframe(&self) -> String {
        self.timeframe.clone()
    }
    pub fn get_window(&self) -> usize {
        self.window
    }
    pub fn get_strategy(&self) -> String {
        self.strategy.clone()
    }
    pub fn get_socket_url(&self) -> String {
        self.socket_url.clone()
    }
    pub fn get_api_url(&self) -> String {
        self.api_url.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Candle {
    timestamp: u64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

impl Candle {
    pub fn new(timestamp: u64, open: f64, high: f64, low: f64, close: f64, volume: f64) -> Candle {
        Candle {
            timestamp,
            open,
            high,
            low,
            close,
            volume,
        }
    }
    // only for testing purposes
    pub fn zeros() -> Candle {
        // Creates candle with all values set to zero
        Candle {
            timestamp: 0,
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 0.0,
            volume: 0.0,
        }
    }
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
    pub fn open(&self) -> f64 {
        self.open
    }
    pub fn high(&self) -> f64 {
        self.high
    }
    pub fn low(&self) -> f64 {
        self.low
    }
    pub fn close(&self) -> f64 {
        self.close
    }
    pub fn volume(&self) -> f64 {
        self.volume
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CandleLine {
    data: Vec<Candle>,
}

impl CandleLine {
    pub fn new() -> CandleLine {
        // Create new empty candleline
        CandleLine {
            data: Vec::<Candle>::new(),
        }
    }
    pub fn get(&self, index: isize) -> Candle {
        // Return candle with given index
        if index >= 0 {
            self.data[index as usize]
        } else {
            self.data[(self.len() as isize - index) as usize]
        }
    }
    pub fn get_range(&mut self, start_index: usize, end_index: usize) -> Vec<Candle> {
        // Return candle with given index
        if start_index > end_index {
            let temp = &mut self.data.clone()[end_index..start_index];
            temp.reverse();
            temp.to_vec()
        } else {
            self.data[start_index..end_index].to_vec()
        }
    }
    pub fn first(&self) -> Candle {
        // Return first candle
        self.data[0]
    }
    pub fn last(&self) -> Candle {
        // Return last candle
        self.data[self.len() - 1]
    }
    pub fn len(&self) -> usize {
        // Return number of elements in candleline / length of candleline
        self.data.len()
    }
    pub fn push(&mut self, kline: Candle) {
        // Add new candle to candleline
        self.data.push(kline);
    }
    pub fn all(&self) -> Vec<Candle> {
        // Reuturns all candleline as vector of candles
        self.data.clone()
    }
    pub fn timestamps(self) -> Vec<u64> {
        // Returns vector containing timestamps from all candles
        self.data.iter().map(|x| x.timestamp).collect::<Vec<u64>>()
    }
    pub fn opens(self) -> Vec<f64> {
        // Returns vector containing open price from all candles
        self.data.iter().map(|x| x.open).collect::<Vec<f64>>()
    }
    pub fn highs(self) -> Vec<f64> {
        // Returns vector containing high price from all candles
        self.data.iter().map(|x| x.high).collect::<Vec<f64>>()
    }
    pub fn lows(self) -> Vec<f64> {
        // Returns vector containing low price from all candles
        self.data.iter().map(|x| x.low).collect::<Vec<f64>>()
    }
    pub fn closes(self) -> Vec<f64> {
        // Returns vector containing close price from all candles
        self.data.iter().map(|x| x.close).collect::<Vec<f64>>()
    }
    pub fn volumes(self) -> Vec<f64> {
        // Returns vector containing volume from all candles
        self.data.iter().map(|x| x.volume).collect::<Vec<f64>>()
    }
    pub fn new_from_vec(candles: Vec<Candle>) -> CandleLine {
        CandleLine { data: candles }
    }
}

#[derive(Debug, std::cmp::PartialEq)]
pub enum Signal {
    Sleep,
    Long,
    Short,
}
/*
#[derive(Debug)]
pub struct Strategy {
    name: String,
    sig_calc: dyn Fn(&Vec<Candle>) -> Signal
}
impl Strategy {
    pub fn new(name:String, sig_calc: dyn Fn(&Vec<Candle>) -> Signal) -> Strategy{
        Strategy {
            name,
            sig_calc
        }
    }
}
*/

#[derive(Debug, Clone, Copy)]
pub struct Market {
    currency_a_amount: f64,
    currency_b_amount: f64,
    ratio_a_to_b: f64,
    ratio_b_to_a: f64,
    transaction_fee: f64,
}
impl Market {
    pub fn new(
        currency_a_amount: f64,
        currency_b_amount: f64,
        ratio_a_to_b: f64,
        transaction_fee: f64,
    ) -> Market {
        let ratio_b_to_a = 1.0 / &ratio_a_to_b;
        Market {
            currency_a_amount,
            currency_b_amount,
            ratio_a_to_b,
            ratio_b_to_a,
            transaction_fee,
        }
    }
    pub fn update_ratio(&mut self, ratio: f64) {
        self.ratio_a_to_b = ratio;
        self.ratio_b_to_a = 1.0 / ratio;
    }
    pub fn set_fee(&mut self, fee: f64) {
        self.transaction_fee = fee;
    }
    pub fn buy(&mut self, amount: f64) {
        if &amount >= &0.0 && &amount * self.ratio_a_to_b <= self.currency_b_amount {
            self.currency_a_amount += amount * (1.0 - self.transaction_fee);
            self.currency_b_amount -= amount * self.ratio_a_to_b;
        } else {
            println!("Given value exceeds allowed range");
        }
    }
    pub fn sell(&mut self, amount: f64) {
        if &amount >= &0.0 && &amount <= &self.currency_a_amount {
            self.currency_a_amount -= amount;
            self.currency_b_amount += amount * self.ratio_a_to_b * (1.0 - self.transaction_fee);
        } else {
            println!("Given value exceeds allowed range");
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn market_buy_sell() {
        let mut market = Market::new(0.0, 100.0, 4.0, 0.001);
        println!("{:#?}", &market);
        market.buy(10.0);
        market.sell(1.0);
        market.sell(9.0);
        market.buy(99999.999);
        market.buy(-123.0);
        market.sell(99999.999);
        market.sell(-123.0);
        println!("{:#?}", &market);
    }
}
