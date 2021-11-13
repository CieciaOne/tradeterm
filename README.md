# tradeterm
Terminal based algo-trading solution

### Things to implement:
- cli interface for config obj
- timeframe transformation
-[x] getting exchange info 
- storing and encryption of keys
- buy/sell max amount or fraction eg. buy coin A with .4 of all owned coin B
- signal into order translation (for ease of use stick to market price?)
- statistics for backtest (as a serializable object?):
	- avg in-position time
	- avg trade gain / loss ABS and %
	- Cumulative gain / loss ABS and %
	- Passive change from start
		In progress, neeed to fix journal not taking whole period lacking newest data?
	- Active change from start
		Getting NaNs lol
	- amount lost on fees in trades


