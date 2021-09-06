# tradeterm
Terminal based multithreaded(tokio) algo-trading solution also called bot in simple terms


#### Preprocessing of data
After getting data from API it is transformed into Vec of Candles, which is a custom type containing fields: [timestamp, open, high, low, close, volumea]

#### Tick processor
Gets a vec of candles of set length and strategy object based on which it generates signal

#### Order manager
Based on this signal order manager creates order
