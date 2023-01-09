// 121. Best Time to Buy and Sell Stock
// Dynamic Programming 32
struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = vec![0;prices.len()+1];
        let mut max = 0;
        dp[0] = std::i32::MAX;
        for i in 0..prices.len() {
            dp[i+1] = dp[i];
            if prices[i] < dp[i] {
                dp[i+1] = prices[i];
            }
            if prices[i] - dp[i+1] > max {
                max = prices[i] - dp[i+1];
            }
        }
        return max;
    }
}
fn main() {
}
