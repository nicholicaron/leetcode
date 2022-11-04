
/*takes an array of stock prices, indexed by the day that price is available
  find max difference (profit) between indexed values (prices on different days)

  12 ms runtime: 91st percentile, 3mb memory usage: 63rd percentile
  95.6% faster than similar Golang solution, while using 66% less memory */
fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = prices[0];
    let mut max_price = 0;
    let mut temp_profit: i32;
    let mut profit: i32 = 0;

    for price in prices {
        if price < min_price {
            min_price = price;
            max_price = price;
        } else if price > max_price {
            max_price = price;
        }

        temp_profit = max_price - min_price;
        if temp_profit > profit {
            profit = temp_profit
        }
    }
    if profit < 0 {0} 
    else {profit}
}



fn main() {
    let prices = vec![7,6,4,3,1];
    println!("{:?}", max_profit(prices));
}
