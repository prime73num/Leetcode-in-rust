// 96. Unique Binary Search Trees
// Dynamic Programming 9
struct Solution {}
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp = vec![0;n as usize+1];
        dp[0] = 1;
        for i in 1..=n {
            let mut sum = 0;
            for j in 1..=i {
                sum += dp[j as usize-1] * dp[(i-j) as usize]
            }
            dp[i as usize] = sum;
        }
        dp[n as usize]
    }
}
fn main() {
}
