// 213. House Robber II
// Dynamic Programming 3-
struct Solution {}
impl Solution {
    pub fn help(nums: &[i32]) -> i32 {
        if nums.is_empty() { return 0;}
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
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 { return nums[0]; }
        let size = nums.len();
        let mut max = Self::help(&nums[1..]);
        let temp = Self::help(&nums[..size-1]);
        if temp > max {
            max = temp;
        }
        return max;
    }
}
fn main() {
}
