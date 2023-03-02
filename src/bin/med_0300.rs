// 300. Longest Increasing Subsequence
// Dynamic Programming 41
struct Solution {}
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0;nums.len()];
        let mut res = 0;
        for i in 0..nums.len() {
            let value = nums[i];
            let mut max = 0;
            for j in 0..i {
                if nums[j] < value && dp[j] > max {
                    max = dp[j];
                }
            }
            dp[i] = max + 1;
            if dp[i] > res { res = dp[i]; }
        }
        return res;
    }
    pub fn length_of_lis_2(nums: Vec<i32>) -> i32 {
        let mut res = 1;
        let mut dp = vec![0;nums.len()];
        dp[0] = 1;
        for i in 1..nums.len() {
            let mut max = 1;
            let temp = nums[i];
            for j in 0..i {
                if nums[j] < temp && dp[j] + 1 > max {
                    max = dp[j] + 1;
                }
            }
            dp[i] = max;
            if max > res {
                res = max;
            }
        }
        dbg!(dp);
        res
    }
}
fn main() {
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2() {
        let tt = vec![0,1,0,3,2,3];
        dbg!(&tt);
        let res = Solution::length_of_lis_2(tt);
        dbg!(res);
    }
}
