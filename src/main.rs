//#![allow(dead_code, unused_doc_comments)]
use reqwest::{self, Client};
//use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};
//use std::time::{Duration, Instant};
use tungstenite::{connect, Message};

use tradeterm::strategy;
use tradeterm::types::{Broker, Candle, Config, Event, Journal, Market, Order, Signal, Stats};
use tradeterm::types::{OrderRespType, OrderSide, OrderType, TimeInForce};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // get information abour exchange

    let binance = Broker::new(
        "BINANCE".to_string(),
        "api_key".to_string(),
        "https://api.binance.com/api/v3/".to_string(),
        "wss://stream.binance.com:9443/ws".to_string(),
    );

    let binance_testnet = Broker::new(
        "BINANCE".to_string(),
        "api_key".to_string(),
        "https://testnet.binance.vision/api".to_string(),
        "wss://testnet.binance.vision/ws".to_string(),
    );

    let config = Config::new(
        "def_cfg".to_string(),
        "This is a default config for development purposes".to_string(),
        "BTCUSDT".to_string(),
        "1m".to_string(),
        32,
        "ExS".to_string(),
        binance,
    );
    // let info = get_sym_info(&config).await.unwrap();
    // println!(
    //     "{:#?}",
    //     info.get("symbols")
    //         .unwrap()
    //         .get(0)
    //         .unwrap()
    //         .get("filters")
    //         .unwrap()
    //         .get(5)
    //         .unwrap()
    //         .get("stepSize")
    //         .unwrap()
    // );
    //let step = info.get("symbols").unwrap(); //.get("filters").unwrap().get(4);
    //let step = info.get("symbols").unwrap().get("filters").unwrap().get(5).unwrap();

    let time = get_server_time(&config).await;
    let test_order = Order::new(
        "BTCUSDT".to_string(),
        OrderSide::BUY,
        OrderType::MARKET,
        None,
        Some(0.0001),
        None,
        None,
        None,
        None,
        None,
        Some(OrderRespType::FULL),
        None,
        time.unwrap(),
    );
    let resp = place_order(&config, test_order).await;
    println!("{:?}", resp);
    //println!("{:?}", time);
    //let mut market = Market::new(0.0, 10000.0, 1.0, 0.0001, 0.0001, 0.001);
    //backtrade(&config, &mut market).await;
    //trade_live(&config).await;
    Ok(())
}

async fn place_order(cfg: &Config, order: Order) -> Result<Value, reqwest::Error> {
    let client = Client::new();
    let payload = json!({
            "symbol":cfg.get_ticker().to_uppercase()});
    let res = client
        .get(cfg.get_api_url() + "exchangeInfo")
        .query(&payload)
        .send()
        .await?
        .text()
        .await?;
    let data: Value = serde_json::from_str(&res).unwrap();
    Ok(data)
}

async fn get_server_time(cfg: &Config) -> Result<u64, reqwest::Error> {
    let client = Client::new();
    //let payload = json!({
    //"symbol":cfg.get_ticker().to_uppercase()});
    let res = client
        .get(cfg.get_api_url() + "time")
        //.query(&payload)
        .send()
        .await?
        .text()
        .await?;
    println!("{:?}", &res);
    // let data: Value = serde_json::from_str(&res);
    // println!("{:?}", data.as_str().unwrap().parse::<u64>().unwrap());
    Ok(1)
}

async fn get_sym_info(cfg: &Config) -> Result<Value, reqwest::Error> {
    let client = Client::new();
    let payload = json!({
            "symbol":cfg.get_ticker().to_uppercase()});
    let res = client
        .get(cfg.get_api_url() + "exchangeInfo")
        .query(&payload)
        .send()
        .await?
        .text()
        .await?;
    let data: Value = serde_json::from_str(&res).unwrap();
    Ok(data)
}

async fn get_ex_info(cfg: &Config) -> Result<Value, reqwest::Error> {
    let client = Client::new();
    let res = client
        .get(cfg.get_api_url() + "exchangeInfo")
        //.query(&payload)
        .send()
        .await?
        .text()
        .await?;
    let data: Value = serde_json::from_str(&res).unwrap();
    Ok(data)
}
// Parse from Value object to matrix of floats
async fn get_candles(cfg: &Config) -> Result<Vec<Candle>, reqwest::Error> {
    let client = Client::new();
    let payload = json!({
        "symbol":cfg.get_ticker().to_uppercase(),"interval":cfg.get_timeframe(),"limit":50});
    let res = client
        .get(cfg.get_api_url() + "klines")
        .query(&payload)
        .send()
        .await?
        .text()
        .await?;

    // Parse from Value object to matrix of floats
    let data: Vec<Vec<Value>> = serde_json::from_str(&res).unwrap();
    let candle_vec = data
        .iter()
        .map(|row| {
            Candle::new(
                row[0].as_u64().unwrap_or(0),
                row[1].as_str().unwrap().parse::<f64>().unwrap_or(0.0),
                row[2].as_str().unwrap().parse::<f64>().unwrap_or(0.0),
                row[3].as_str().unwrap().parse::<f64>().unwrap_or(0.0),
                row[4].as_str().unwrap().parse::<f64>().unwrap_or(0.0),
                row[5].as_str().unwrap().parse::<f64>().unwrap_or(0.0),
            )
        })
        .collect::<Vec<Candle>>();
    Ok(candle_vec)
}

fn process(candles: &Vec<Candle>, strategy_name: String) -> Signal {
    let signal = match strategy_name.to_lowercase().as_str() {
        "exs" => strategy::exs(candles),
        _ => Signal::Sleep,
    };
    //println!("Last candle:\n{:#?} \nSignal: \t{:#?}", &candles.last(),&signal);
    signal
}

async fn backtrade(cfg: &Config, market: &mut Market) {
    let candles = get_candles(&cfg).await.unwrap();
    let mut signals: Vec<Signal> = vec![];

    let mut journal = Journal::new();

    for index in 0..candles.len() {
        market.update_ratio(candles.get(index).unwrap().close());
        if &cfg.get_window() > &candles.len() {
            signals.push(process(&candles.to_vec(), cfg.get_strategy()));
        } else {
            if index + &cfg.get_window() <= candles.len() {
                signals.push(process(
                    &candles[index..&cfg.get_window() + index].to_vec(),
                    cfg.get_strategy(),
                ));
            }
        }
        match signals.last().unwrap() {
            Signal::Long => market.buy(market.b_in_a()),
            Signal::Short => market.sell(market.a_in_b()),
            Signal::Sleep => (),
        }

        let e = Event::new(
            candles[index].timestamp() as usize,
            signals.last().unwrap().clone(),
            market.clone(),
            candles[index],
        );
        journal.put(e);
    }

    let mut stats = Stats::init();
    stats.calculate(journal);
    println!("{:#?}", stats);
    println!(
        "Final result{:#?}\nFirst C: {:#?}\nLast C: {:#?}",
        &market.a_in_b(),
        &candles.first(),
        &candles.last()
    );
}

fn socket_sub_payload(cfg: &Config) -> String {
    let payload = json!({"method":"SUBSCRIBE",
    "params":[format!("{}@kline_{}",cfg.get_ticker().to_lowercase(),cfg.get_timeframe())],
    "id":1});
    serde_json::to_string(&payload).unwrap()
}

async fn trade_live(cfg: &Config, market: &Market) {
    let mut candles = get_candles(&cfg).await.unwrap();

    let (mut socket, _response) = connect(cfg.get_socket_url()).expect("Cannot connect");

    let payload = socket_sub_payload(&cfg);

    socket.write_message(Message::Text(payload.into())).unwrap();

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
                    let signal: Signal;
                    if &cfg.get_window() > &candles.len() {
                        signal = process(&candles.to_vec(), cfg.get_strategy());
                    } else {
                        signal = process(
                            &candles[&candles.len() - cfg.get_window()..].to_vec(),
                            cfg.get_strategy(),
                        );
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
