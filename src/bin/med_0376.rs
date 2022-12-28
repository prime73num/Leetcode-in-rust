















// 376. Wiggle Subsequence
// Greedy 3
struct Solution {}
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut trend = 0;
        for i in 0..nums.len() {
            if i == nums.len()-1 {
                sum += 1;
            } else {
                let mut new_trend = 0;
                if nums[i+1] > nums[i] {
                    new_trend = 1;
                } else if nums[i+1] < nums[i] {
                    new_trend = -1;
                }
                if new_trend!=0 && new_trend != trend {
                    sum += 1;
                    trend = new_trend;
                }
            }
        }
        sum
    }
}
fn main() {
}
