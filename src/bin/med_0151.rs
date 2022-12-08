










// 151. Reverse Words in a String
// String 4


struct Solution {}
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut res = String::new();
        let bytes = s.as_bytes();
        let mut end = (bytes.len()-1) as i32;
        for i in (0..bytes.len()).rev() {
            if bytes[i] == b' ' {
                if end > i as i32 {
                    let str = std::str::from_utf8(&bytes[i+1..end as usize+1]).unwrap();
                    res.push_str(str);
                    res.push(' ');
                }
                end = i as i32 -1;
            }
        }
        if end >= 0 {
            let str = std::str::from_utf8(&bytes[0..end as usize+1]).unwrap();
            res.push_str(str);
            res.push(' ');
        }
        res.pop();
        res
    }
}

fn main() {
    let res = Solution::reverse_words("   this  is a   test  ".to_string());
    println!("{}", res);
}
