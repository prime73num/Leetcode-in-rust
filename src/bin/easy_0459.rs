















// 459. Repeated Substring Pattern
// String 7








struct Solution {}
impl Solution {
    fn get_prefix_array(s: &str) -> Vec<usize> {
        let s = s.as_bytes();
        let mut res = vec![0;s.len()];
        for i in 1..s.len() {
            let mut prefix = i;
            res[i] = 0;
            while prefix != 0 {
                prefix = res[prefix-1];
                if s[prefix] == s[i] {
                    res[i] = prefix + 1;
                    break;
                }
            }
        }
        res
    }
    fn str_str(haystack: String, needle: String) -> i32 {
        let prefix_array = Self::get_prefix_array(&needle);
        let s = haystack.as_bytes();
        let n = needle.as_bytes();
        let mut match_count = 0;
        let mut point = 0;
        while point != s.len() {
            if n[match_count] == s[point] {
                point += 1;
                match_count += 1;
            } else {
                if match_count != 0 {
                    match_count = prefix_array[match_count-1];
                } else {
                    point += 1;
                }
            }
            if match_count == n.len() {
                return (point - n.len()) as i32;
            }
        }
        -1
    }
    pub fn repeated_substring_pattern(s: String) -> bool {
        let mut long_str = Vec::new();
        long_str.extend_from_slice(&s.as_bytes()[1..]);
        long_str.extend_from_slice(&s.as_bytes()[..s.len()-1]);
        let long_str = String::from_utf8(long_str).unwrap();
        println!("{}", long_str);
        return Self::str_str(long_str, s) != -1;
    }
}

fn main() {
    let res = Solution::repeated_substring_pattern("abab".to_string());
    println!("{:?}", res);
}
