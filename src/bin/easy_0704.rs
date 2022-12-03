









// Binary search 
// Array 2



struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut s, mut e) = (0, nums.len());
        while s != e {
            let mid = (s+e -1) / 2;
            let temp = nums[mid];
            if temp == target {
                return mid as i32;
            } else if temp > target {
                e = mid;
            } else {
                s = mid +1;
            }
        }
        return -1;
    }
}
fn main() {
}
