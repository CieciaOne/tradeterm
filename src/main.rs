#![allow(dead_code, unused_doc_comments)]
use reqwest::{self, Client};
use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};
use std::time::{Duration, Instant};
use tungstenite::{connect, Message};

//mod strategy;

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
    let config = Config::new(
        "def_cfg".to_string(),
        "This is a default config for development purposes".to_string(),
        "BTCUSDT".to_string(),
        "5m".to_string(),
        16,
        0,
        "ExS".to_string(),
    );

    let candles = get_candles(&config).await?;

    trade_live(&config, candles).await;

    Ok(())
}

async fn get_candles(config: &Config) -> Result<Vec<Candle>, reqwest::Error> {
    let client = Client::new();
    let payload = json!({
        "symbol":config.get_ticker().to_uppercase(),"interval":config.get_timeframe(),"limit":500});
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

fn process_ticks(candles: &Vec<Candle>, strategy_name:String) -> Signal {

    println!("last:{:#?}", &candles);

    let signal: Signal = Signal::Sleep;
    signal
}

async fn trade_live(cfg: &Config, mut candles: Vec<Candle>) {
    // Use prefetched Candles
    // Connecting to websocket API and then processing data from it to generate signals

    let (mut socket, response) =
        connect("wss://stream.binance.com:9443/ws").expect("Cannot connect");

    // Status display
    println!("Connected with status: {}", response.status());
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    // Create payload to subscribe to websocket
    let payload = json!({"method":"SUBSCRIBE",
    "params":[format!("{}@kline_{}",cfg.get_ticker().to_lowercase(),cfg.get_timeframe())],
    "id":1});
    let payload_text = serde_json::to_string(&payload).unwrap();

    socket.write_message(Message::Text(payload_text.into())).unwrap();
    loop {
        match socket.read_message().expect("Error reading message") {
            Message::Text(t) => {
                let msg: serde_json::Value = serde_json::from_str(&t).unwrap();
                // debug message
                // println!("{:?}",&msg);

                let candle = &msg["k"];

                // Only if Message is correct (no errors)
                if msg.get("e") != None {
                    let t_new = Instant::now();
                    // Creating new candle from data acquired
                    let new_candle = Candle::new(
                        candle["t"].as_u64().unwrap(),
                        candle["o"].as_str().unwrap().parse::<f64>().unwrap(),
                        candle["h"].as_str().unwrap().parse::<f64>().unwrap(),
                        candle["l"].as_str().unwrap().parse::<f64>().unwrap(),
                        candle["c"].as_str().unwrap().parse::<f64>().unwrap(),
                        candle["v"].as_str().unwrap().parse::<f64>().unwrap(),
                    );
                    if candle.get("x") == Some(&Value::Bool(true)) {
                        // On full candle premanently add to candles vec
                        candles.push(new_candle);
                    } else {
                        // Replace latest tick with new one
                        candles.pop();
                        candles.push(new_candle);
                    }

                    // Run processing function on range of candles
                    let mut signal: Signal;
                    if &cfg.get_window() > &candles.len() {
                        signal = process_ticks(&candles.to_vec(),cfg.get_strategy());
                    } else {
                        signal =
                            process_ticks(&candles[&candles.len() - cfg.get_window()..].to_vec(), cfg.get_strategy());
                    }
                    //println!("{:?}",&signal);
                    //println!("Message processing took: {} microseconds",t_new.elapsed().as_micros());
                } else {
                    println!("{:?}", msg.get("e"));
                }
            }
            _ => (),
        };
    }
}

