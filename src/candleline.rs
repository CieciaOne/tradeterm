// This file is temporary place for Candleline struct make it easier to edit main
#[derive(Serialize, Deserialize, Debug, Clone)]
struct CandleLine {
    data: Vec<Candle>,
}

impl Iterator for CandleLine {
    type Item = Candle;

    //please don't use this, you will regret immediately
    fn next(&mut self) -> Option<Candle> {
        Some(*self.data.iter().next()?)
    }
}
impl CandleLine {
    pub fn new() -> CandleLine {
        /// Create new empty candleline
        CandleLine {
            data: Vec::<Candle>::new(),
        }
    }
    pub fn get(&self, index: usize) -> Candle {
        /// Return candle with given index
        self.data[index]
    }
    pub fn get_range(&mut self, start_index: usize, end_index: usize) -> Vec<Candle> {
        /// Return candle with given index
        if start_index > end_index {
            let temp = &mut self.data.clone()[end_index..start_index];
            temp.reverse();
            temp.to_vec()
        } else {
            self.data[start_index..end_index].to_vec()
        }
    }
    pub fn last(&self) -> Candle {
        /// Return last candle
        self.data[self.len()]
    }
    pub fn len(&self) -> usize {
        /// Return number of elements in candleline / length of candleline
        self.data.len()
    }
    pub fn push(&mut self, kline: Candle) {
        /// Add new candle to candleline
        self.data.push(kline);
    }
    pub fn all(&self) -> Vec<Candle> {
        /// Reuturns all candleline as vector of candles
        self.data.clone()
    }
    pub fn timestamps(self) -> Vec<u64> {
        /// Returns vector containing timestamps from all candles
        self.data.iter().map(|x| x.timestamp).collect::<Vec<u64>>()
    }
    pub fn opens(self) -> Vec<f64> {
        /// Returns vector containing open price from all candles
        self.data.iter().map(|x| x.open).collect::<Vec<f64>>()
    }
    pub fn highs(self) -> Vec<f64> {
        /// Returns vector containing high price from all candles
        self.data.iter().map(|x| x.high).collect::<Vec<f64>>()
    }
    pub fn lows(self) -> Vec<f64> {
        /// Returns vector containing low price from all candles
        self.data.iter().map(|x| x.low).collect::<Vec<f64>>()
    }
    pub fn closes(self) -> Vec<f64> {
        /// Returns vector containing close price from all candles
        self.data.iter().map(|x| x.close).collect::<Vec<f64>>()
    }
    pub fn volumes(self) -> Vec<f64> {
        /// Returns vector containing volume from all candles
        self.data.iter().map(|x| x.volume).collect::<Vec<f64>>()
    }
    pub fn from_vec(kline: Vec<Candle>) -> CandleLine {
        let mut res = CandleLine::new();
        kline.iter().for_each(|x| {
            res.push(Candle {
                timestamp: x.timestamp,
                open: x.open,
                high: x.high,
                low: x.low,
                close: x.close,
                volume: x.volume,
            });
        });
        res
    }
    pub fn heikinashi(&self) -> CandleLine {
        /// Method generating Heikin Ashi candlesticks, best use with buffer of at least 10
        /// candles in past to decrease "synthetic" first candle
        let data = self.all();
        let mut ha = CandleLine::new();
        ha.push(Candle::new(
            data[0].timestamp(),
            (data[0].open() + data[0].close()) / 2.0,
            data[0].high(),
            data[0].low(),
            (data[0].open() + data[0].high() + data[0].low() + data[0].close()) / 4.0,
            data[0].volume(),
        ));

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
