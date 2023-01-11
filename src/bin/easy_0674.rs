// 674. Longest Continuous Increasing Subsequence
// Dynamic Programming 42
struct Solution {}
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0;nums.len()];
        let mut res = 1;
        dp[0] = 1;
        for i in 1..nums.len() {
            if nums[i-1] < nums[i] {
                dp[i] = dp[i-1] + 1;
            } else {
                dp[i] = 1;
            }
            if dp[i] > res {
                res = dp[i];
            }
        }
        return res;
    }
}
fn main() {
}
