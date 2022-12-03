









 // Minimum Size Subarray Sum
 // Array 5




struct Solution {}
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut res = std::i32::MAX;
        let mut s = 0;
        let mut e = 0;
        let mut sum = 0;
        while s!=nums.len() {
            while sum < target && e < nums.len() {
                sum += nums[e];
                e += 1;
            }
            if sum < target {
                break;
            } else {
                if e - s < res as usize {
                    res = (e-s) as i32;
                }
            }
            sum -= nums[s];
            s += 1;
        }
        if res == std::i32::MAX {
            res = 0;
        }
        res
    }
}




fn main() {
}
