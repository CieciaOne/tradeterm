
struct Strategy {
    name: String,
    sig_calc: dyn Fn(&Vec<Candle>) -> Signal
}
impl Strategy {
    pub fn new(name:String, sig_calc: Fn(&Vec<Candle>)) - Strategy{
        Strategy {
            name,
            sig_calc
        }
    }
}

pub fn get_strategies() -> Vec<Strategy> {
    vec!(

        )
}

fn exs(candles: &Vec<Candle>) -> Signal {

}
