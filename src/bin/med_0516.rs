// 516. Longest Palindromic Subsequence
// Dynamic Programming 53
struct Solution {}
impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let mut reverse = s.clone().into_bytes();
        reverse.reverse();
        let s2 = String::from_utf8(reverse).unwrap();
        let bytes1 = s.as_bytes();
        let bytes2 = s2.as_bytes();
        let mut dp = vec![0;bytes1.len()+1];
        for i in 0..bytes2.len() {
            let mut old = 0;
            let val = bytes2[i];
            for i in 1..dp.len() {
                let temp = dp[i];
                if dp[i-1] > dp[i] {
                    dp[i] = dp[i-1];
                }
                if val == bytes1[i-1] {
                    dp[i] = old + 1;
                }
                old = temp;
            }
        }
        return dp[bytes1.len()];
    }
}
fn main() {
}
