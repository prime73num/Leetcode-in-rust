















// Intersection of Two Arrays
// Hash Map 4





struct Solution {}
impl Solution {
    pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::hash_set::HashSet;
        let mut map = HashSet::new();
        let mut res = Vec::new();
        nums1.sort();
        nums2.sort();
        nums1.dedup();
        nums2.dedup();
        if nums1.len() < nums2.len() {
            for i in nums1 {
                map.insert(i);
            }
            for i in nums2 {
                if map.contains(&i) {
                    res.push(i);
                }
            }
        } else {
            for i in nums2 {
                map.insert(i);
            }
            for i in nums1 {
                if map.contains(&i) {
                    res.push(i);
                }
            }
        }
        res
    }
}

fn main() {
}
