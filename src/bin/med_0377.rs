// 377. Combination Sum IV
// Dynamic Programming 21
struct Solution {}
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0;target as usize + 1];
        dp[0] = 1;
        for i in 0..=target {
            for value in &nums {
                if i >= *value {
                    dp[i as usize] += dp[(i-value) as usize];
                }
            }
        }
        return dp[target as usize];
    }
}
fn main() {
}
