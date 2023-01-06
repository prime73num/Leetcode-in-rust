// 62. Unique Paths
// Dynamic Programming 6
struct Solution {}
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec![0;n as usize];
        dp[0] = 1;
        for _ in 0..m {
            for j in 1..n {
                dp[j as usize] = dp[j as usize] + dp[j as usize-1];
            }
        }
        return dp[n as usize-1];
    }
}
fn main() {
}
