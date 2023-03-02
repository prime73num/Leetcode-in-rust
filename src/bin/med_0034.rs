









// 34. Find First and Last Position of Element in Sorted Array



struct Solution;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1,-1];
        }
        let mut res = Vec::new();
        let mut s = 0;
        let mut e = nums.len()-1;
        while s<e {
            let mid = (s+e)/2;
            if nums[mid] < target {
                s = mid + 1;
            } else {
                e = mid;
            }
        }
        res.push(s as i32);
        let mut s = 0;
        let mut e = nums.len()-1;
        while s<e {
            let mid = (s+e+1)/2;
            if nums[mid] <= target {
                s = mid;
            } else {
                e = mid-1;
            }
        }
        res.push(s as i32);
        if nums[res[1] as usize] != target {
            res[0] = -1;
            res[1] = -1;
        }
        res
    }
}
fn main() {
}
