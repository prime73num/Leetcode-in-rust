struct Solution {}
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut pre = 0;
        let mut times = 0;
        for i in nums {
            if times == 0 {
                pre = i;
                times = 1;
                continue;
            }
            if i == pre {
                times += 1;
            } else {
                times -= 1;
            }
        }
        return pre;
    }
}
fn main() {
    Solution::majority_element(vec![0,4]);
}
