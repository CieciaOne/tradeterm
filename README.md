# Tradeterm
Terminal based algo-trading solution.

## Description
Tired of neverending loses? Now you can lose even more and without knowing about it.
This is a framework, which lets you design and run your own trading strategies based on candleline datastreams. It's main goal is to give the end user as much freedom as possible. While also providing them with easy to use interface.
## How to use
For now, you don't. This is not working build, but if you really want to, then clone the repo, write your strategy in strategy.rs, include it in process fn in main.rs and you can figure the rest on your own. No, really better don't use it.
## Features
- Config - main configuration used for running a session  
- Broker - configuration for Exchange's APIs
- trade_live() - live strategy runtime, connects via websockets and 

### Things to implement:
- cli interface
- timeframe transformation e.g. you fetch 1m candles and then transform them into 1h or something else. This way you are not limited to the few standard ones, like 15m, 1h, etc.
-[x] getting exchange info 
- storage and encryption of keys
- buy/sell max amount or fraction eg. buy coin A with .4 of all owned coin B
- signal into order translation (for ease of use stick to market price?)
- statistics for backtest (remember about serialization and display):
	- avg in-position time
	- avg trade gain / loss ABS and %
	- Cumulative gain / loss ABS and %
	- [x] Passive change from start
	- [x] Active change from start
	- amount lost on fees in trades