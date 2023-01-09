// 198. House Robber
// Dynamic Programming 29
struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0;nums.len()+1];
        dp[1] = nums[0];
        for i in 1..nums.len() {
            let temp = nums[i];
            let idx = i + 1;
            let mut max = dp[idx-2] + temp;
            if dp[idx-1] > max {
                max = dp[idx-1];
            }
            dp[idx] = max;
        }
        return dp[nums.len()];
    }
}
fn main() {
}
