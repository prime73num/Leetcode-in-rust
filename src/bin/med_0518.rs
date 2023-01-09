// 518. Coin Ch/ange II
// Dynamic Programming 19
struct Solution {}
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0;amount as usize + 1];
        dp[0] = 1;
        for i in 0..coins.len() {
            let value = coins[i];
            for i in value..=amount {
                dp[i as usize] += dp[(i-value) as usize];
            }
        }
        return dp[amount as usize];
    }
}
fn main() {
}
