













// 15. 3Sum
// Hash Map 9




struct Solution {}
impl Solution {
    fn unique(x: usize, y: usize, z: usize, nums:&[i32]) -> bool {
        if x > 0 && nums[x-1] == nums[x] {
            return false;
        } 
        if x < y -1 && nums[y-1] == nums[y] {
            return false;
        }
        if y < z -1 && nums[z-1] == nums[z] {
            return false;
        }
        true
    }
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res = Vec::new();
        let size = nums.len();
        for i in 0..size {
            let mut s = i+1;
            let mut e = size-1;
            let temp = nums[i];
            while s < e {
                let sum = nums[s] + nums[e];
                if sum == -temp {
                    if Self::unique(i, s, e, &nums) {
                        res.push(vec![nums[i], nums[s], nums[e]]);
                    }
                    e = e - 1;
                } else if sum < -temp {
                    s = s + 1;
                } else {
                    e = e - 1;
                }
            }
        }
        res
    }
}

fn main() {
}
