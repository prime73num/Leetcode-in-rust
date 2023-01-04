// 714. Best Time to Buy and Sell Stock with 
// Greedy 23
struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut sum = 0;
        let mut min = prices[0];
        let mut max = prices[0];
        for i in 0..prices.len() {
            let temp = prices[i];
            if max >= temp + fee {
                if max > min + fee {
                    sum += max - min - fee;
                }
                max = temp;
                min = temp
            }
            if temp > max { max = temp; }
            if temp < min { min = temp; }
        }
        if max > min + fee {
            sum += max - min - fee;
        }
        sum
    }
}
fn main() {
}
