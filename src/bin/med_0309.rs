// 309. Best Time to Buy and Sell Stock with Cooldown
// Dynamic Programming 37
struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = vec![[0,0];prices.len()+1];
        dp[1][0] = 0;
        dp[1][1] = -prices[0];
        for i in 2..prices.len()+1 {
            let value = prices[i-1];
            let first = value + dp[i-1][1];
            let second = dp[i-2][0] - value;
            dp[i][0] = dp[i-1][0];
            dp[i][1] = dp[i-1][1];
            if dp[i][0] < first { dp[i][0] = first; }
            if dp[i][1] < second { dp[i][1] = second; }
        }
        return dp[prices.len()][0];
    }
}
fn main() {
}
