








// 454. 4Sum II
// Hash Map 7





struct Solution {}
impl Solution {
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        use std::collections::hash_map::HashMap;
        let mut map = HashMap::new();
        for i in &nums1 {
            for j in &nums2 {
                *map.entry(i+j).or_insert(0) += 1;
            }
        }
        let mut res = 0;
        for i in &nums3 {
            for j in &nums4 {
                let target = -(*i+*j);
                if map.contains_key(&target) {
                    res += map[&target];
                }
            }
        }
        res
    }
}

fn main() {
}
