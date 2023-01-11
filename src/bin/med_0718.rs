// 718. Maximum Length of Repeated Subarray
// Dynamic Programming 43
struct Solution {}
impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![0;nums1.len()+1];
        let mut res = 0;
        for idx in 0..nums2.len() {
            let value = nums2[idx];
            for i in (1..dp.len()).rev() {
                dp[i] = 0;
                if value == nums1[i-1] {
                    dp[i] = dp[i-1] + 1;
                }
                if dp[i] > res {
                    res = dp[i];
                }
            }
        }
        return res;
    }
}
fn main() {
}
