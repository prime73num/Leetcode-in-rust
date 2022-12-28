
















// 122. Best Time to Buy and Sell Stock II
// Greedy 6
struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut pre = std::i32::MAX;
        let mut trend = 0;
        for i in 0..prices.len() {
            if i == prices.len()-1 {
                if prices[i] > pre {
                    sum += prices[i] - pre;
                }
            } else {
                let mut new_trend = 0;
                if prices[i+1] > prices[i] {
                    new_trend = 1;
                } else if prices[i+1] < prices[i] {
                    new_trend = -1;
                }
                if new_trend!=0 && new_trend != trend {
                    if prices[i] > pre {
                        sum += prices[i] - pre;
                    }
                    pre = prices[i];
                    trend = new_trend;
                }
            }
        }
        sum
    }
}
fn main() {
}
