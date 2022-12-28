



















// 1005. Maximize Sum Of Array After K Negations
// Greedy 9
struct Solution {}
impl Solution {
    pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut min = 0;
        let size = nums.len();
        for _ in 0..k {
            nums[min] = -nums[min];
            if min != size-1 && nums[min+1] < nums[min] {
                min += 1;
            }
        }
        let mut sum = 0;
        for i in nums {
            sum += i;
        }
        sum
    }
}
fn main() {
}
