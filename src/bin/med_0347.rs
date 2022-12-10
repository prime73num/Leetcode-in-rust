

















// 347. Top K Frequent Elements
// Stack and HHeap 8


struct Solution {}
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::hash_map::HashMap;
        use std::collections::BinaryHeap;
        let mut map = HashMap::new();
        let mut min_heap = BinaryHeap::new();
        let mut res = vec![0;k as usize];
        for i in nums {
            *map.entry(i).or_insert(0) += 1;
        }
        for (key, value) in map {
            if min_heap.len() < k as usize {
                min_heap.push((std::cmp::Reverse(value), key));
            } else {
                let min = min_heap.peek().unwrap().0.0;
                if value > min {
                    min_heap.pop();
                    min_heap.push((std::cmp::Reverse(value), key));
                }
            }
        }
        for i in (0..k).rev() {
            res[i as usize] = min_heap.pop().unwrap().1;
        }
        res
    }
}

fn main() {
}
