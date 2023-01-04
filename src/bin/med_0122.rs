
















// 122. Best Time to Buy and Sell Stock II
// Greedy 6
struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max = prices[0];
        let mut min = prices[0];
        for i in 0..prices.len() {
            let temp = prices[i];
            if temp < max {
                if max > min {
                    sum += max - min;
                }
                min = temp;
                max = temp
            }
            if max < temp {max = temp;}
            if min > temp {min = temp;}
        }
        if max > min {
            sum += max - min;
        }
        sum
    }
}
fn main() {
}
