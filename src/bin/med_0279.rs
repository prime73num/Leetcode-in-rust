// Dynamic Programming 24
struct Solution {}
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = vec![-1;n as usize + 1];
        dp[0] = 0;
        for i in 0..=n {
            let mut min = std::i32::MAX;
            let mut value = 1;
            while value * value <= i {
                let temp = dp[(i-value*value) as usize];
                if temp != -1 && temp+1 < min {
                    min = temp + 1;
                }
                value += 1;
            }
            if min != std::i32::MAX {
                dp[i as usize] = min;
            }
        }
        return dp[n as usize];
    }
}
fn main() {
}
