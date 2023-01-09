// 188. Best Time to Buy and Sell Stock IV
// Dynamic Programming 36
struct Solution {}
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let mut dp = vec![0;prices.len()];
        let mut sell_or_buy = -1;
        let size = prices.len();
        let sum = -prices[0];
        dp[0] = 0;
        let mut old = 0;
        for _ in 1..=2*k {
            old = dp[0];
            dp[0] = sum - dp[0];
            for j in 1..size {
                let value = prices[j] * sell_or_buy;
                let mut new = old + value;
                if dp[j-1] > new { new = dp[j-1]; }
                old = dp[j];
                dp[j] = new;
            }
            sell_or_buy = -sell_or_buy;
        }
        return dp[size-1];
    }
}
fn main() {
    let testcase = vec![2,4,1];
    Solution::max_profit(2, testcase);
}
