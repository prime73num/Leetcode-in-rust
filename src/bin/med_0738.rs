// 738. Monotone Increasing Digits
// Greedy 22
struct Solution {}
impl Solution {
    fn get(mut n: i32) -> Vec<i32> {
        let mut res = Vec::new();
        while n > 0 {
            res.push(n%10);
            n = n/10;
        }
        res.reverse();
        res
    }
    fn add(nums: &[i32]) -> i32 {
        let mut res = 0;
        let mut tens = 1;
        for i in (0..nums.len()).rev() {
            res += tens * nums[i];
            tens = tens*10;
        }
        res
    }
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        if n==0 {
            return n;
        }
        let nums = Self::get(n);
        let mut res = vec![0;nums.len()];
        let mut idx = 0;
        let mut pre = nums[0];
        for i in 0..nums.len() {
            if nums[i] < pre { break; }
            if nums[i] > pre {
                idx = i;
                pre = nums[i];
            }
            res[i] = nums[i];
            if i==nums.len()-1 {
                return n;
            }
        }
        res[idx] -= 1;
        for i in idx+1..res.len() {
            res[i] = 9;
        }
        return Self::add(&res);
    }
}
fn main() {
}
