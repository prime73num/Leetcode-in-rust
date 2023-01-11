// 1035. Uncrossed Lines
// Dynamic Programming 45
struct Solution {}
impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![0;nums1.len()+1];
        for i in 0..nums2.len() {
            let mut old = 0;
            let val = nums2[i];
            for i in 1..dp.len() {
                let temp = dp[i];
                if dp[i-1] > dp[i] {
                    dp[i] = dp[i-1];
                }
                if val == nums1[i-1] {
                    dp[i] = old + 1;
                }
                old = temp;
            }
        }
        return dp[nums1.len()];
    }
}
fn main() {
}
