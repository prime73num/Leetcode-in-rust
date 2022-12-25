











// 90. Subsets II
// BackTrace 13
struct Solution {}
impl Solution {
    fn backtrace(pos: &mut Vec<i32>, res:&mut Vec<Vec<i32>>, nums: &[i32], count: &[i32]) {
        if nums.is_empty() {
            res.push(pos.clone());
            return;
        }
        let size = pos.len();
        let head = nums[0];
        for _ in 0..=count[0] {
            Self::backtrace(pos, res, &nums[1..], &count[1..]);
            pos.push(head);
        }
        pos.truncate(size);
    }
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map = std::collections::hash_map::HashMap::new();
        for i in nums {
            *map.entry(i).or_insert(0) += 1;
        }
        let mut num = Vec::new();
        let mut count = Vec::new();
        for i in map.iter() {
            num.push(*i.0);
            count.push(*i.1);
        }
        let mut res = Vec::new();
        let mut pos = Vec::new();
        Self::backtrace(&mut pos, &mut res, &num, &count);
        res
    }
}
fn main() {
}
