// 416. Partition Equal Subset Sum
// Dynamic Programming 13
struct Solution {}
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut sum = 0;
        let size = nums.len();
        for i in 0..size {
            sum += nums[i];
        }
        if sum %2 != 0 { return false;}
        let target = sum/2;
        let mut dp = vec![false;target as usize + 1];
        dp[0] = true;
        for i in 0..size {
            let temp = nums[i];
            for j in (temp..=target).rev() {
                if !dp[j as usize] && dp[(j-temp) as usize] {
                    dp[j as usize] = true;
                }
            }
        }
        return dp[target as usize];
    }
}
fn main() {
    let testcase = [1,2,5];
    Solution::can_partition(testcase.to_vec());
}
