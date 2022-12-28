

















// 53. Maximum Subarray
// Greedy 4
struct Solution {}
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max = std::i32::MIN;
        for i in nums {
            if sum > 0 {
                sum = sum + i;
            } else {
                sum = i;
            }
            if sum > max { max = sum;}
        }
        max
    }
}
fn main() {
}
