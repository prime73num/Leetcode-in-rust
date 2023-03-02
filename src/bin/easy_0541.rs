







// 541. Reverse String II
// String 2





struct Solution {}
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut start = 0;
        let mut end = k as usize;
        let mut s = s.into_bytes();
        let size = s.len();
        while start < size {
            if end > size {
                end = size;
            }
            let sum = start + end;
            for i in start..(start + end)/2 {
                let temp = s[i];
                s[i] = s[sum-i-1];
                s[sum-1-i] = temp;
            }
            start += 2*k as usize;
            end += 2*k as usize;
        }
        String::from_utf8(s).unwrap()
    }
}

fn main() {
    println!("Hello");
}
