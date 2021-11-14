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
    pub fn get(&self, index: usize) -> Candle {
        // Return candle with given index
        self.data[index]
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
        // Returns all candleline as vector of candles
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

#[derive(Debug, std::cmp::PartialEq, Clone)]
pub enum Signal {
    Sleep,
    Long,
    Short,
}

#[derive(Debug, Clone)]
pub struct Journal {
    entries: Vec<Event>,
}

impl Journal {
    pub fn new() -> Journal {
        Journal {
            entries: Vec::new(),
        }
    }
    pub fn put(&mut self, event: Event) {
        self.entries.push(event);
    }
    pub fn get(&self,index: usize) -> Event {
        self.entries[index].clone()
    }
    pub fn get_all(&self) -> Vec<Event> {
        self.entries.clone()
    }
    pub fn get_timestamps(&self) -> Vec<usize> {
        self.clone()
            .get_all()
            .iter()
            .map(|x| x.get_timestamp())
            .collect()
    }
    pub fn get_signals(&self) -> Vec<Signal> {
        self.clone().get_all().iter().map(|x| x.get_signal()).collect()
    }
    pub fn get_candles(&self) -> Vec<Candle> {
        self.clone().get_all().iter().map(|x| x.get_candle()).collect()
    }
    pub fn get_markets(&self) -> Vec<Market> {
        self.clone().get_all().iter().map(|x| x.get_market()).collect()
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    timestamp: usize,
    signal: Signal,
    market: Market,
    candle: Candle,
}
impl Event {
    pub fn new(timestamp: usize, signal: Signal, market: Market, candle: Candle) -> Event {
        Event {
            timestamp,
            signal: signal.clone(),
            market: market.clone(),
            candle: candle.clone(),
        }
    }
    pub fn get_timestamp(&self) -> usize {
        self.timestamp
    }
    pub fn get_signal(&self) -> Signal {
        self.signal.clone()
    }
    pub fn get_market(&self) -> Market {
        self.market.clone()
    }
    pub fn get_candle(&self) -> Candle {
        self.candle
    }
}
#[derive(Debug)]
pub struct Stats {
    chg_passive: f64,
    chg_active: f64,
    avg_in_pos: f64,
    avg_gain: f64,
    avg_loss: f64,
    cum_gain: f64,
    cum_loss: f64,
    cum_fees: f64,
}
impl Stats {
    pub fn init() -> Stats {
        Stats {
            chg_passive: 0.0,
            chg_active: 0.0,
            avg_in_pos: 0.0,
            avg_gain: 0.0,
            avg_loss: 0.0,
            cum_gain: 0.0,
            cum_loss: 0.0,
            cum_fees: 0.0,
        }
    }

    fn calc_chg_p(&mut self, journal: Journal) {
        let data = journal.get_all();
        let f = data.first();
        let l = data.last();
        //println!("\n========={:#?} {:#?}\n=========", f, l);
        self.chg_passive =
            (l.unwrap().get_candle().close() - f.unwrap().get_candle().open()) / l.unwrap().get_candle().close()
    }
    fn calc_chg_a(&mut self, journal: Journal) {
        let f = *journal.get_markets().first().unwrap();
        let l = *journal.get_markets().last().unwrap();

        if l.get_b_amount() == 0.0 {
            println!("=0 -> {0} - {1} + {2} / {1}",l.get_b_amount(), f.get_b_amount(),l.a_in_b());
            self.chg_active = (l.get_b_amount() - f.get_b_amount() + l.a_in_b()) / f.get_b_amount();
        } else {
            println!("!=0 -> {0} - {1} / {1}",l.get_b_amount(), f.get_b_amount());
            self.chg_active = (l.get_b_amount() - f.get_b_amount()) / f.get_b_amount();
        }
    }
    fn calc_avg_in_pos(&mut self,journal: Journal) {
    }
    fn calc_avg_gain(&mut self,journal: Journal) {
    }
    fn calc_avg_loss(&mut self,journal: Journal) {
    }
    fn calc_cum_gain(&mut self,journal: Journal) {
    }
    fn calc_cum_loss(&mut self,journal: Journal) {
    }
    fn calc_cum_fees(&mut self,journal: Journal) {
    }

    pub fn calculate(&mut self, journal: Journal) {
        self.calc_chg_a(journal.clone());
        self.calc_chg_p(journal.clone());
        self.calc_avg_in_pos(journal.clone());
        self.calc_avg_gain(journal.clone());
        self.calc_avg_loss(journal.clone());
        self.calc_cum_gain(journal.clone());
        self.calc_cum_loss(journal.clone());
        self.calc_cum_fees(journal.clone());
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Market {
    currency_a_amount: f64,
    currency_b_amount: f64,
    ratio_a_to_b: f64,
    ratio_b_to_a: f64,
    min_a_transaction: f64,
    min_b_transaction: f64,
    transaction_fee: f64,
}
impl Market {
    pub fn new(
        currency_a_amount: f64,
        currency_b_amount: f64,
        ratio_a_to_b: f64,
        min_a_transaction: f64,
        min_b_transaction: f64,
        transaction_fee: f64,
    ) -> Market {
        let ratio_b_to_a = 1.0 / &ratio_a_to_b;
        Market {
            currency_a_amount,
            currency_b_amount,
            ratio_a_to_b,
            ratio_b_to_a,
            min_a_transaction,
            min_b_transaction,
            transaction_fee,
        }
    }
    pub fn get_a_amount(&self) -> f64 {
        self.currency_a_amount
    }
    pub fn get_b_amount(&self) -> f64 {
        self.currency_b_amount
    }
    pub fn a_in_b(&self) -> f64 {
        self.currency_a_amount * self.ratio_a_to_b
    }
    pub fn b_in_a(&self) -> f64 {
        self.currency_b_amount * self.ratio_b_to_a
    }

    pub fn update_ratio(&mut self, ratio: f64) {
        self.ratio_a_to_b = ratio;
        self.ratio_b_to_a = 1.0 / ratio;
    }
    pub fn set_fee(&mut self, fee: f64) {
        self.transaction_fee = fee;
    }
    fn min_a_transaction(&self) -> f64 {
        self.min_a_transaction
    }
    fn min_b_transaction(&self) -> f64 {
        self.min_b_transaction
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
    pub fn buy_max(&mut self) {
        let t = self.min_a_transaction() * (self.b_in_a() / self.min_a_transaction()).floor();
    }
    pub fn sell_max(&mut self) {
        if self.get_a_amount() >= self.min_a_transaction() {
            self.sell(self.get_a_amount());
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

    #[test]
    fn conversion() {
        let mut market = Market::new(0.0, 100.0, 4.0, 0.001);

        println!("\nA={}, A in B={}", market.get_a_amount(), market.a_in_b());

        println!("B={}, B in A={}", market.get_b_amount(), market.b_in_a());
    }
}
