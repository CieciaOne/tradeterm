import numpy as np
import pandas as pd
from binance.client import Client
from binance.enums import *
import pandas_ta as ta

import datetime
import websocket
import json

from flask import Flask, render_template
import plotly
import plotly.graph_objects as go


### Read cfg
cfg = open("keys").read().splitlines()
API_KEY = cfg[0]
API_SECRET_KEY = cfg[1]

### API setup
client = Client(API_KEY, API_SECRET_KEY)
BASE_URL = 'https://api.binance.com/api'
BASE_TEST_URL = 'https://testnet.binance.vision/api'
WSS_URL = 'wss://stream.binance.com:9443/ws'

### Developer options
websocket.enableTrace(True)
pd.set_option('display.max_columns', None)

### Globals
tickers = []
tickers_map = {}
buff_limit = 10

### Strategies
Strat = ta.Strategy(
        name="HA+Cross",
        ta=[
            {"kind": "ha"},
            {"kind": "wma", "length":2},
            {"kind": "wma", "length":2,"suffix":"shifted"},
            ]
        )

### Web Interface
app = Flask(__name__,template_folder='./')

@app.route('/')
def home():
    return "Homeland"

@app.route('/<symbol>')
def symbol(symbol):
    global Strat,tickers_map
    symbols  = get_symbol_list()
    if symbol in symbols:
        print(symbol.upper())
#    if df["WMA_2_shifted"].exists:
#        df["WMA_2_shifted"] = df["WMA_2_shifted"].shift(1)
    #return render_template('index.html',symbols=symbols)
    return f'Ola {symbol}'

def get_symbol_list():
    info = client.get_exchange_info()
    symbols = []
    for s in info["symbols"]:
        symbols.append(s["symbol"])
    return symbols

def main():
    global tickers, tickers_map
    for ticker in tickers:
        tickers_map[ticker] = pd.DataFrame(columns=['datetime','open','high','low','close','volume'])
    print("this be main")

def apply_strategy(df,strategy):
    df.set_index(pd.DatetimeIndex(df["datetime"]),inplace=True)
    df.ta.strategy(strategy,timed=True)
    df.drop("datetime",axis=1,inplace=True)
    return df

#    fig = go.Figure(data=go.Candlestick(x=df.index,
#        open=df["open"],
#        high=df["high"],
#        low=df["low"],
#        close=df["close"]))
#    fig.show()
#    graphJSON = json.dumps(fig, cls=plotly.utils.PlotlyJSONEncoder)

    #print(df)

def cli():
    global tickers, tickers_map
    ticker = input("Tickers\n").upper()
    while len(ticker)>=1:
        tickers.append(ticker)
        ticker = input().upper()

def get_last_klines(tickers,interval,limit=500):
    kline = np.array(client.get_klines(symbol=ticker,interval=interval,limit=limit),dtype=float)
    t, o, h, l, c, v = kline[:,6],kline[:,1],kline[:,2],kline[:,3],kline[:,4],kline[:,5],
    to_datetime = lambda t: datetime.datetime.fromtimestamp((int(t)+1)/1000)
    t =  np.array([to_datetime(ti) for ti in t])
    data = np.array([t,o,h,l,c,v])
    data = np.rot90(data)
    data = np.flip(data,0)
    temp = pd.DataFrame(data,columns = ['datetime','open','high','low','close','volume'] )
    return temp

def get_past_klines(symbol,interval,time="1 day ago"):
    kline = np.array(client.get_historical_klines(symbol,interval,time),dtype=float)
    t, o, h, l, c, v = kline[:,6],kline[:,1],kline[:,2],kline[:,3],kline[:,4],kline[:,5],
    to_datetime = lambda t: datetime.datetime.fromtimestamp((int(t)+1)/1000)
    t =  np.array([to_datetime(ti) for ti in t])
    data = np.array([t,o,h,l,c,v])
    data = np.rot90(data)
    data = np.flip(data,0)
    temp = pd.DataFrame(data,columns = ['datetime','open','high','low','close','volume'] )
    return temp

### WEBSOCKETS
def on_open(ws):
    global tickers_map
    tickers = list(tickers_map.keys())
    for s in range(len(tickers)):
        tickers[s]=tickers[s].lower()+'@kline_1m'
    payload = json.dumps({
        "method": "SUBSCRIBE",
        "params": tickers,
        "id": 1
        })
    print(payload)
    ws.send(payload)

def on_close(ws):
    print('Connection closed')

def on_error(ws, error):
    print(f'Shutting down {error}')

def on_message(ws, message):
    global tickers_map, buff_limit
    json_message = json.loads(message)
    #print(json_message)
    candle = json_message['k']
    is_candle_closed = candle['x']
    timestamp = datetime.datetime.fromtimestamp(int(candle['t'])/1000)
    data = tickers_map[json_message["s"]]
    temp = pd.DataFrame([[timestamp,candle["o"],candle["h"],candle["l"],candle["c"],candle["v"]]], columns=['datetime','open','high','low','close','volume'],dtype=float)
    #temp = temp.round({'open':2,'high':2,'low':2,'close':2})
    if is_candle_closed:
        data = data.append(temp)
        tickers_map[json_message["s"]] = data
    else:
        tickers_map[json_message["s"]].loc[-1] = temp
        print(temp,tickers_map[json_message["s"]])
    if len(tickers_map[json_message["s"]]) > buff_limit:
        tickers_map[json_message["s"]] = tickers_map[json_message["s"]].tail(buff_limit)
    #print(tickers_map[json_message["s"]])

def get_live():
    ws = websocket.WebSocketApp(WSS_URL, on_open=on_open,on_error=on_error, on_close=on_close, on_message=on_message)
    ws.run_forever()


#if __name__ == "__main__":
#    main()


df = get_past_klines("BTCUSDT","1m","3 hour ago")
tickers_map["BTCUSDT"] = df
print(tickers_map)
get_live()
