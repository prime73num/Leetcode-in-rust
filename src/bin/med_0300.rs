// 300. Longest Increasing Subsequence
// Dynamic Programming 41
struct Solution {}
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0;nums.len()];
        let mut res = 0;
        for i in 0..nums.len() {
            let value = nums[i];
            let mut max = 0;
            for j in 0..i {
                if nums[j] < value && dp[j] > max {
                    max = dp[j];
                }
            }
            dp[i] = max + 1;
            if dp[i] > res { res = dp[i]; }
        }
        return res;
    }
}
fn main() {
}
