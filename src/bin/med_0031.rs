// 31. Next Permutation
struct Solution {}
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let size = nums.len();
        let mut pre = nums[size-1];
        let mut idx = size-1;
        for i in (0..size).rev() {
            if nums[i] < pre {
                break;
            }
            pre = nums[i];
            idx = i;
        }
        if idx == 0 {
            nums.reverse();
            return;
        }
        let target = nums[idx - 1];
        let mut start = idx;
        let mut end = size-1;
        while start < end {
            let mid = (start + end+1)/2;
            if nums[mid] > target {
                start = mid;
            } else {
                end = mid-1;
            }
        }
        nums.swap(idx-1, start);
        nums[idx..].reverse();
        return;
    }
}

fn main() {
}

