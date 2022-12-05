













// 1. Two Sum
// Hash Map 6






struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::hash_set::HashSet;
        let mut map = HashSet::new();
        let mut res = Vec::new();
        for i in 0..nums.len() {
            let temp = target - nums[i];
            if map.contains(&temp) {
                res.push(i as i32);
                let idx = nums.iter().enumerate().find(|(_,y)| {
                    return **y == temp;
                }).unwrap().0;
                res.push(idx as i32);
                break;
            }
            map.insert(nums[i]);
        }
        res
    }
}
fn main() {
}
