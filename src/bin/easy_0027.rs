













// Remove Element
// Array 3




struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut pos = 0;
        let mut p = 0;
        while p!= nums.len() {
            if nums[p] != val {
                let temp = nums[p];
                nums[p] = nums[pos];
                nums[pos] = temp;
                pos += 1;
            }
            p += 1;
        }
        nums.shrink_to(pos);
        pos as i32
    }
}
fn main() {
}
