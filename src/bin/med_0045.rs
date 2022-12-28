



















// 45. Jump Game II
// Greedy 8
struct Solution {}
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut max_idx = 0;
        let mut sum = 0;
        let mut pre = 0;
        let mut p = 0;
        while p<nums.len() {
            if p>pre {
                pre = max_idx as usize;
                sum += 1;
            }
            let new_idx = p+nums[p] as usize;
            if new_idx > max_idx {
                max_idx = new_idx;
            }
            p += 1;
        }
        return sum;
    }
}
fn main() {
}
