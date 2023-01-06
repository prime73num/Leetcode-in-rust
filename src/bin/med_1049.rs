// 1049. Last Stone Weight II
// Dynamic Programming 14
struct Solution {}
impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in &stones {
            sum += *i;
        }
        let target = sum/2;
        let mut dp = vec![false; target as usize + 1];
        dp[0] = true;
        for i in &stones {
            for j in (*i..=target).rev() {
                if !dp[j as usize] {
                    dp[j as usize] = dp[(j-*i) as usize];
                }
            }
        }
        for i in (0..=target).rev() {
            if dp[i as usize] {
                return sum - 2*i;
            }
        }
        return 0;
    }
}
fn main() {
    let test = vec![2,7,4,8,1,1];
    Solution::last_stone_weight_ii(test);
}
