// 494. Target Sum
// Dynamic Programming 16
struct Solution {}
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, mut target: i32) -> i32 {
        for i in &nums {
            target += *i;
        }
        if target < 0 { return 0;}
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;
        let size = nums.len();
        for i in 0..size {
            let temp = nums[i];
            for j in (2*temp..=target).rev() {
                dp[j as usize] = dp[j as usize] + dp[(j-2*temp) as usize];
            }
        }
        return dp[target as usize];
    }
}
fn main() {
    let testcase = vec![100];
    let res = Solution::find_target_sum_ways(testcase, -200);
    println!("{}", res);
}
