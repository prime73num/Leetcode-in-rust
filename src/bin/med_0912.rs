







// 912, Sort an array


use rand::prelude::*;
struct Solution {}
impl Solution {
    fn sort(arr: &mut [i32]) {
        if arr.is_empty() {return;}
        let size = arr.len();
        let idx = rand::thread_rng().gen_range(0..size);
        arr.swap(idx, size-1);
        let temp = arr[size-1];
        let mut start = 0;
        for i in 0..size {
            if arr[i] < temp {
                arr.swap(i, start);
                start += 1;
            }
        }
        arr.swap(size-1, start);
        Self::sort(&mut arr[..start]);
        Self::sort(&mut arr[start+1..]);
    }
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        Self::sort(&mut nums);
        nums
    }
}
fn main() {
}

#[cfg(test)]
mod name {
    use super::*;
    #[test]
    fn test_quicksort() {
        let test = vec![3,6,5,66,77,9,023,99,4,1,22,9989];
        let mut sorted = test.clone();
        sorted.sort();
        let res = Solution::sort_array(test);
        assert_eq!(res, sorted);

        dbg!(res);
    }
}
