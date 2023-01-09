
















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
    pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
        let mut dp = vec![[0,0];prices.len()];
        dp[0][0] = 0;
        dp[0][1] = -prices[0];
        for i in 1..prices.len() {
            let value = prices[i];
            let first = value + dp[i-1][1];
            let second = dp[i-1][0] - value;
            dp[i][0] = dp[i-1][0];
            dp[i][1] = dp[i-1][1];
            if dp[i][0] < first { dp[i][0] = first; }
            if dp[i][1] < second { dp[i][1] = second; }
        }
        return dp[prices.len()-1][0];
    }
}
fn main() {
}
