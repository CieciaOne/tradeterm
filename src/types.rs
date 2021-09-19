use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    name: String,
    description: String,
    ticker: String,
    timeframe: String,
    window: usize,
    shift: i32,
    strategy: String,
}
impl Config {
    pub fn new(
        name: String,
        description: String,
        ticker: String,
        timeframe: String,
        window: usize,
        shift: i32,
        strategy: String,
    ) -> Config {
        Config {
            name: name.to_lowercase(),
            description,
            ticker,
            timeframe,
            window,
            shift,
            strategy: strategy.to_lowercase(),
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
    pub fn get_shift(&self) -> i32 {
        self.shift
    }
    pub fn get_strategy(&self) -> String {
        self.strategy.clone()
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
        /// Creates candle with all values set to zero
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
