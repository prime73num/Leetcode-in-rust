// 322. Coin Change
// Dynamic Programming 23
struct Solution {}
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![-1;amount as usize + 1];
        dp[0] = 0;
        for i in 0..=amount {
            let mut min = std::i32::MAX;
            for value in &coins {
                if i >= *value {
                    let temp = dp[(i-*value) as usize];
                    if temp != -1 && temp+1 < min {
                        min = temp + 1;
                    }
                }
            }
            if min != std::i32::MAX {
                dp[i as usize] = min;
            }
        }
        return dp[amount as usize];
    }
}
fn main() {
}
