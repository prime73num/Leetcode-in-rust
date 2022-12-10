








// 239. Sliding Window Maximum
// Stack and HHeap 7


struct Solution {}
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut window:VecDeque<(i32,i32)> = VecDeque::new();
        let mut res = Vec::new();
        for i in 0..k {
            let temp = nums[i as usize];
            while !window.is_empty() && window.back().unwrap().0 < temp {
                window.pop_back();
            }
            window.push_back((temp,i));
        }
        res.push(window.front().unwrap().0);
        for i in k..nums.len() as i32 {
            if window.front().unwrap().1 < i-k+1{
                window.pop_front();
            }
            let temp = nums[i as usize];
            while !window.is_empty() && window.back().unwrap().0 < temp {
                window.pop_back();
            }
            window.push_back((temp,i));
            res.push(window.front().unwrap().0);
        }
        res
    }
}
fn main() {
    let test = vec![1,-1];
    let res = Solution::max_sliding_window(test, 1);
    println!("{:?}", res);
}
