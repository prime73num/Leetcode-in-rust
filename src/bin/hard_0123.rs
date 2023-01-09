// 123. Best Time to Buy and Sell Stock III
// Dynamic Programming 35
struct Solution {}
impl Solution {
    pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
        let k = 2;
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
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut pre_max = 0;
        let mut min = std::i32::MAX;
        let mut part_min = std::i32::MAX;
        let mut total_max = std::i32::MIN;
        for i in 0..prices.len() {
            let value = prices[i];
            if value < min { min = value; }
            if value < part_min { part_min = value; }
            let temp_max = pre_max - value;
            if temp_max > total_max {
                total_max = temp_max;
            }
            let total = total_max + value;
            if total > res { res = total; }
            let temp = value - min;
            if temp > pre_max {
                pre_max = temp;
                part_min = std::i32::MAX;
            }
        }
        if pre_max > res {
            res = pre_max;
        }
        res
    }
}
fn main() {
}
