









// 18.43Sum
// Hash Map 9







struct Solution {}

impl Solution {
    fn unique(x: isize, y: isize, nums: &[i32]) -> bool {
        if x < y -1 && nums[y as usize-1] == nums[y as usize] {
            return false;
        }
        true
    }
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res = Vec::new();
        let size = nums.len();
        for i in 0..size {
            if !Self::unique(-1, i as isize, &nums) { continue; }
            for j in i+1..size {
                if !Self::unique(i as isize, j as isize, &nums) { continue; }
                let mut s = j+1;
                let mut e = size-1;
                let temp = nums[i] + nums[j] - target;
                while s < e {
                    let sum = nums[s] + nums[e];
                    if sum == -temp {
                        if Self::unique(j as isize, s as isize, &nums)
                            && Self::unique(s as isize, e as isize, &nums) {
                            res.push(vec![nums[i], nums[j], nums[s], nums[e]]);
                        }
                        e = e - 1;
                    } else if sum < -temp {
                        s = s + 1;
                    } else {
                        e = e - 1;
                    }
                }
            }
        }
        res
    }
}
fn main() {
}
