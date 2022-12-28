


















// 55. Jump Game        
// Greedy 7
struct Solution {}
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_idx = 0;
        let mut p = 0;
        while p<nums.len() {
            if p as i32 > max_idx {
                return false;
            }
            let new_idx = p as i32 +nums[p];
            if new_idx > max_idx {
                max_idx = new_idx;
            }
            p += 1;
        }
        return true;
    }
}
fn main() {
}
