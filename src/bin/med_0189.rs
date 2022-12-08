










// 189. Rotate Array
// String 5




struct Solution {}
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let size = nums.len();
        let mut val = nums[0];
        let mut next_idx = k % size as i32;
        let mut min = next_idx;
        while next_idx != 0 {
            let temp = nums[next_idx as usize];
            nums[next_idx as usize] = val;
            if next_idx < min {
                min = next_idx;
            }
            val = temp;
            next_idx = (next_idx + k as i32) % size as i32;
        }
        nums[0] = val;
        for i in 1..min {
            let mut val = nums[i as usize];
            let mut next_idx = i + k;
            while next_idx != i {
                let temp = nums[next_idx as usize];
                nums[next_idx as usize] = val;
                val = temp;
                next_idx = (next_idx + k as i32) % size as i32;
            }
            nums[i as usize] = val;
        }
    }
}

fn main() {
    let mut test = vec![1,2,3,4];
    Solution::rotate(&mut test, 2);
    println!("{:?}", test);
}
