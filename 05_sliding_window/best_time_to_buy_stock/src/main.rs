use std::cmp::max;
fn max_profit(prices: &Vec<u32>) -> u32 {
    let len = prices.len();
    if len < 2 {return 0;}

    let mut l = 0;
    let mut r = 1;
    let mut curr_max: u32 = 0;
    
    while r < len { 
        if prices[r] > prices[l] {
            curr_max = max(curr_max, prices[r] - prices[l]);
        }

        if prices[r] < prices[l] {
            l = r;
        }

        r += 1;
    }
    curr_max
}
fn main() {
    let mut prices = vec![10, 1, 5, 6, 7, 1];
    println!("{}", max_profit(&prices));
    prices = vec![10, 8, 7, 5, 2];
    println!("{}", max_profit(&prices));
}
