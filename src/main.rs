//#![allow(dead_code, unused_doc_comments)]
use reqwest::{self, Client};
//use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};
//use std::time::{Duration, Instant};
use tungstenite::{connect, Message};

use tradeterm::types::{Config,Candle,Signal};
use tradeterm::strategy;

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
        .map(|row| Candle::new(
             row[0].as_u64().unwrap_or(0),
             row[1].as_str().unwrap().parse::<f64>().unwrap_or(0.0),
             row[2].as_str().unwrap().parse::<f64>().unwrap_or(0.0),
             row[3].as_str().unwrap().parse::<f64>().unwrap_or(0.0),
             row[4].as_str().unwrap().parse::<f64>().unwrap_or(0.0),
             row[5].as_str().unwrap().parse::<f64>().unwrap_or(0.0),
        ))
        .collect::<Vec<Candle>>();
    Ok(candle_vec)
}

fn process_ticks(candles: &Vec<Candle>, strategy_name:String) -> Signal {
    let signal = match strategy_name.to_lowercase().as_str(){
        "exs" => strategy::exs(candles),
         _ => Signal::Sleep
    };
    //println!("Last candle:\n{:#?} \nSignal: \t{:#?}", &candles.last(),&signal);
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
                    // Measure time of processing
                    // let t_new = Instant::now();
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

