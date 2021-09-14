#![allow(dead_code, unused_doc_comments)]
use reqwest::{self, Client};
use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};
use std::time::{Duration, Instant};
use tungstenite::{connect, Message};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    name: String,
    description: String,
    ticker: String,
    timeframe: String,
    window: usize,
    shift: i32,
    strategy: String,
}
impl Config {
    pub fn new(name: String, description: String, ticker: String, timeframe: String, window: usize, shift: i32, strategy: String) -> Config {
        Config {
            name,
            description,
            ticker,
            timeframe,
            window,
            shift,
            strategy
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
struct Candle {
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
enum Signal {
    Sleep,
    Long,
    Short,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    //let data = live_kline("BTCUSDT".to_string(), "5m".to_string()).await;
    let config = Config::new("def_cfg".to_string(),"This is a default config for development purposes".to_string(),"BTCUSDT".to_string(),"5m".to_string(),16,0,"ExS".to_string());
    let payload = json!({
        "symbol":config.get_ticker().to_uppercase(),"interval":config.get_timeframe(),"limit":500});

    let klines = get_candles(payload).await?;

    
    // println!("{:#?}", klines);
    trade_live(config, klines).await;
    //let candles = CandleLine::from_vec(klines);
    //let sigs = generate_signals(candles.clone()).await;
    //backtest(ticker.to_string(), candles.clone(), sigs, 100.0, 0.002).await;

    Ok(())
}
/*
async fn generate_signals(data: CandleLine) -> Vec<Signal> {
    let mut signals: Vec<Signal> = Vec::new();
    let d = data.clone().closes();
    let ha = data.heikinashi();
    let ma1 = moving_average(ha.clone().closes().clone(), 4);
    let mut ma2 = moving_average(ha.clone().closes().clone(), 4);
    ma2.rotate_right(2);
    signals.push(Signal::Sleep);
    let mut entry_price = 0.0;

    // Yo bro I fucked up i'ma mess not gonna lie
    let mut last_trade_idx = 0;
    let limit = 3; //trade frequency limiter, sets number of bars that needs to pass between changing positions

    //

    let mut position = Signal::Sleep;
    for i in 1..ma1.len() {
        if i - last_trade_idx > limit
            && crossover(&ma1[i - 1..i + 1].to_vec(), &ma2[i - 1..i + 1].to_vec())
        {
            signals.push(Signal::Long);
            position = Signal::Long;
            entry_price = d[i];
            last_trade_idx = i;
        } else if i - last_trade_idx > limit
            && crossover(&ma2[i - 1..i + 1].to_vec(), &ma1[i - 1..i + 1].to_vec())
        {
            signals.push(Signal::Short);
            position = Signal::Short;
            entry_price = d[i];
            last_trade_idx = i;
        } else {
            signals.push(Signal::Sleep);
        }
    }
    signals
    //println!("{:#?}\n{:#?}", ma1, ma2);
}
async fn backtest(
    ticker: String,
    data: CandleLine,
    signal: Vec<Signal>,
    start_balance: f64,
    fee: f64,
) {
    // make a virtual portfolio with current pair and keep track of transactions
    // don't forget fees

    let mut folio: [f64; 2] = [0.0, start_balance];

    println!("Backtest for {}", &ticker.to_uppercase());
    //println!("{:#?}", signal);
    if signal.len() != data.len() {
        println!("Your data and signals don't align");
        return;
    }
    let len = signal.len();
    let data_vec = data.all();
    let fee_multiplier = 1.0 - fee;
    //let diffs:Vec<f64> = Vec::new();
    let mut entry_price = 0.0;
    let mut entry_position = Signal::Sleep;
    for sig_id in 0..len {
        println!();
        let price = data_vec[sig_id].close();

        // display percentage change from entry
        match signal[sig_id] {
            Signal::Sleep => match entry_position {
                Signal::Long => {
                    println!(
                        "Pass\tDiff: {:.2} %",
                        (price - entry_price) / entry_price * 100.0
                    );
                }
                Signal::Short => {
                    println!(
                        "Pass\tDiff: {:.2} %",
                        (price - entry_price) / entry_price * (-100.0)
                    );
                }

                _ => {}
            },
            Signal::Long => {
                entry_position = Signal::Long;
                println!("Long");
                entry_price = price;
                if folio[1] != 0.0 {
                    folio[0] = folio[1] / price * fee_multiplier;
                    folio[1] = 0.0;
                }
            }
            Signal::Short => {
                entry_position = Signal::Short;
                entry_price = price;
                println!("Short");
                if folio[0] != 0.0 {
                    folio[1] = folio[0] * price * fee_multiplier;
                    folio[0] = 0.0;
                }
            }
        }

        //println!("V_wallet A:{:.3} \t V_wallet B:{:.3}", folio[0], folio[1]);
    }
    let res;
    if folio[1] != 0.0 {
        res = folio[1]
    } else {
        res = folio[0] * data_vec.last().unwrap().close() * fee_multiplier;
    }
    println!(
        "Result:{:.3}({:.3}%) of starting balance:{:3}",
        res,
        res / start_balance * 100.0,
        start_balance
    );
}
*/

async fn get_candles(payload: Value) -> Result<Vec<Candle>, reqwest::Error> {
    let client = Client::new();
    let res = client
        .get("https://api.binance.com/api/v3/klines")
        .query(&payload)
        .send()
        .await?
        .text()
        .await?;

    // Parse from Value object to matrix of floats
    let data: Vec<Vec<Value>> = serde_json::from_str(&res).unwrap();
    let candle_vec = data
        .iter()
        .map(|row| Candle {
            timestamp: row[0].as_u64().unwrap_or(0),
            open: row[1].as_str().unwrap().parse::<f64>().unwrap_or(0.0),
            high: row[2].as_str().unwrap().parse::<f64>().unwrap_or(0.0),
            low: row[3].as_str().unwrap().parse::<f64>().unwrap_or(0.0),
            close: row[4].as_str().unwrap().parse::<f64>().unwrap_or(0.0),
            volume: row[5].as_str().unwrap().parse::<f64>().unwrap_or(0.0),
        })
        .collect::<Vec<Candle>>();
    Ok(candle_vec)
}

fn process_ticks(candles: &Vec<Candle> /*strategy:*/) -> Signal{
    /// This function takes a vector of candles and strategy hashmap or alternative data structure 
    /// and uses it to process this particullar tick
    /// it is meant to be used in live and backtest scenarios
    /// main focus speed and reliability
    //println!("first:{:?}",candles.first());
    println!("last:{:#?}",&candles);


    let signal: Signal = Signal::Sleep;
    signal

}

async fn trade_live(cfg: Config, mut candles: Vec<Candle>) {
    // Use prefetched Candles
    // Connecting to websocket API and then processing data from it to generate signals

    let (mut socket, response) =
        connect("wss://stream.binance.com:9443/ws").expect("Cannot connect");
    println!("Connected with status: {}", response.status());
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }
    // Create payload to subscribe to websocket
    let payload = json!({"method":"SUBSCRIBE",
    "params":[format!("{}@kline_{}",cfg.get_ticker().to_lowercase(),cfg.get_timeframe())],
    "id":1});
    let a = serde_json::to_string(&payload).unwrap();

    socket.write_message(Message::Text(a.into())).unwrap();
    loop {
        match socket.read_message().expect("Error reading message") {
            Message::Text(t) => {
                let msg: serde_json::Value = serde_json::from_str(&t).unwrap();
                // message debug below
                // println!("{:?}",&msg);
                let kline = &msg["k"];
                // Only if Message is correct (no errors)
                if msg.get("e") != None {
                    let t_new = Instant::now();
                    // Creating new candle from data acquired 
                    let res = Candle::new(
                        kline["t"].as_u64().unwrap(),
                        kline["o"].as_str().unwrap().parse::<f64>().unwrap(),
                        kline["h"].as_str().unwrap().parse::<f64>().unwrap(),
                        kline["l"].as_str().unwrap().parse::<f64>().unwrap(),
                        kline["c"].as_str().unwrap().parse::<f64>().unwrap(),
                        kline["v"].as_str().unwrap().parse::<f64>().unwrap(),
                    );
                    if kline.get("x") == Some(&Value::Bool(true)) {
                        // On full candle premanently add to candles vec
                        candles.push(res);
                    } else {
                        // Replace latest tick with new one
                        candles.pop();
                        candles.push(res);
                        //println!("{:?}",&candles.last());
                    }
                    /*  Strategies Here 
                     *   |
                     *   |
                     *  \ /
                     *   v
                     */

                    // Run processing function on range of candles
                    let mut signal:Signal;
                    if &cfg.get_window() > &candles.len() {
                        signal = process_ticks(&candles.to_vec());
                    }
                    else{
                        signal = process_ticks(&candles[&candles.len()-cfg.get_window()..].to_vec());
                    }
                    //println!("{:?}",&signal);
                    //println!("Message processing took: {} microseconds",t_new.elapsed().as_micros());
                }
                else {
                    println!("{:?}",msg.get("e"));
                }
            
            }
            // Pass everything that is not text data
            _ => (),
        };
        //println!("{:#?}", &candles);
    }
}

// ma and crossover functions will be moved from here
fn moving_average(data: Vec<f64>, window: usize) -> Vec<f64> {
    let mut res = Vec::new();
    if window >= data.len() {
        println!("Functions will return zeros, because window is greater than vector length");
    }
    if window >= 1 {
        for _o in 0..window - 1 {
            res.push(0.0);
        }
        for i in (window - 1)..data.len() {
            res.push(data[(i - (window - 1))..i + 1].iter().sum::<f64>() / window as f64);
        }
    } else {
        println!("Cannot use window size below !");
    }
    res
}

fn crossover<T: std::cmp::PartialOrd>(a: &Vec<T>, b: &Vec<T>) -> bool {
    if a[a.len() - 1] > b[b.len() - 1] && a[a.len() - 2] < b[b.len() - 2] {
        true
    } else {
        false
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cross_positive() {
        let a = vec![2, 4];
        let b = vec![3, 1];
        let c = vec![0, 10, 2, 4];
        assert!(crossover(&a, &b));
        assert!(crossover(&c, &b));
    }

    #[test]
    fn cross_negative() {
        let a = vec![2, 4];
        let b = vec![3, 4];
        let c = vec![0, 10, 2, 4];
        assert!(!crossover(&a, &b));
        assert!(!crossover(&a, &c));
        assert!(!crossover(&b, &c)); // b goes under c not above
    }

    #[test]
    fn ma_check() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = vec![0.0, 1.5, 2.5, 3.5, 4.5, 5.5]; //correct result of moving average
        assert_eq!(b, moving_average(a, 2));
    }
    #[test]
    fn ma_check_non_lin() {
        let a = vec![2.0, 4.0, 6.0, 8.0, 10.0, 0.0, 2.0];
        let b = vec![0.0, 0.0, 4.0, 6.0, 8.0, 6.0, 4.0]; //correct result of moving average
        assert_eq!(b, moving_average(a, 3));
    }

    #[test]
    fn vec_slicing() {
        let a = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(vec!(2, 3, 4), a[1..4]);
    }
}
