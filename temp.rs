
async fn backtest(
    ticker: String,
    data: CandleLine,
    signal: Vec<Signal>,
    start_balance: f64,
    fee: f64,
) {
    // make a virtual portfolio with current pair and keep track of transactions
    // don't forget fees

    let mut folio: [f64; 2] = [0.0, start_balance];

    println!("Backtest for {}", &ticker.to_uppercase());
    //println!("{:#?}", signal);
    if signal.len() != data.len() {
        println!("Your data and signals don't align");
        return;
    }
    let len = signal.len();
    let data_vec = data.all();
    let fee_multiplier = 1.0 - fee;
    //let diffs:Vec<f64> = Vec::new();
    let mut entry_price = 0.0;
    let mut entry_position = Signal::Sleep;
    for sig_id in 0..len {
        println!();
        let price = data_vec[sig_id].close();

        // display percentage change from entry
        match signal[sig_id] {
            Signal::Sleep => match entry_position {
                Signal::Long => {
                    println!(
                        "Pass\tDiff: {:.2} %",
                        (price - entry_price) / entry_price * 100.0
                    );
                }
                Signal::Short => {
                    println!(
                        "Pass\tDiff: {:.2} %",
                        (price - entry_price) / entry_price * (-100.0)
                    );
                }

                _ => {}
            },
            Signal::Long => {
                entry_position = Signal::Long;
                println!("Long");
                entry_price = price;
                if folio[1] != 0.0 {
                    folio[0] = folio[1] / price * fee_multiplier;
                    folio[1] = 0.0;
                }
            }
            Signal::Short => {
                entry_position = Signal::Short;
                entry_price = price;
                println!("Short");
                if folio[0] != 0.0 {
                    folio[1] = folio[0] * price * fee_multiplier;
                    folio[0] = 0.0;
                }
            }
        }

        //println!("V_wallet A:{:.3} \t V_wallet B:{:.3}", folio[0], folio[1]);
    }
    let res;
    if folio[1] != 0.0 {
        res = folio[1]
    } else {
        res = folio[0] * data_vec.last().unwrap().close() * fee_multiplier;
    }
    println!(
        "Result:{:.3}({:.3}%) of starting balance:{:3}",
        res,
        res / start_balance * 100.0,
        start_balance
    );
}


//////////////////////////////////////////////////////////////////////////////////////////


// ma and crossover functions will be moved from here
fn moving_average(data: Vec<f64>, window: usize) -> Vec<f64> {
    let mut res = Vec::new();
    if window >= data.len() {
        println!("Functions will return zeros, because window is greater than vector length");
    }
    if window >= 1 {
        for _o in 0..window - 1 {
            res.push(0.0);
        }
        for i in (window - 1)..data.len() {
            res.push(data[(i - (window - 1))..i + 1].iter().sum::<f64>() / window as f64);
        }
    } else {
        println!("Cannot use window size below !");
    }
    res
}

fn crossover<T: std::cmp::PartialOrd>(a: &Vec<T>, b: &Vec<T>) -> bool {
    if a[a.len() - 1] > b[b.len() - 1] && a[a.len() - 2] < b[b.len() - 2] {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cross_positive() {
        let a = vec![2, 4];
        let b = vec![3, 1];
        let c = vec![0, 10, 2, 4];
        assert!(crossover(&a, &b));
        assert!(crossover(&c, &b));
    }

    #[test]
    fn cross_negative() {
        let a = vec![2, 4];
        let b = vec![3, 4];
        let c = vec![0, 10, 2, 4];
        assert!(!crossover(&a, &b));
        assert!(!crossover(&a, &c));
        assert!(!crossover(&b, &c)); // b goes under c not above
    }

    #[test]
    fn ma_check() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = vec![0.0, 1.5, 2.5, 3.5, 4.5, 5.5]; //correct result of moving average
        assert_eq!(b, moving_average(a, 2));
    }
    #[test]
    fn ma_check_non_lin() {
        let a = vec![2.0, 4.0, 6.0, 8.0, 10.0, 0.0, 2.0];
        let b = vec![0.0, 0.0, 4.0, 6.0, 8.0, 6.0, 4.0]; //correct result of moving average
        assert_eq!(b, moving_average(a, 3));
    }

    #[test]
    fn vec_slicing() {
        let a = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(vec!(2, 3, 4), a[1..4]);
    }
}
