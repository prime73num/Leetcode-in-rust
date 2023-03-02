use std::collections::BinaryHeap;
struct MedianFinder {
    former: BinaryHeap<i32>,
    latter: BinaryHeap<std::cmp::Reverse<i32>>,
    mid :Option<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn test(&self) {
        println!("{:?}", self.former);
        println!("{:?}", self.mid);
        println!("{:?}", self.latter);
    }

    /** initialize your data structure here. */
    fn new() -> Self {
        Self { former: BinaryHeap::new(), latter: BinaryHeap::new(), mid: None}
    }
    
    fn add_num(&mut self, num: i32) {
        if self.mid.is_none() && self.former.is_empty() {
            self.mid = Some(num);
            return;
        }
        if let Some(temp) = self.mid {
            let mut a = temp;
            let mut b = num;
            if num < temp {
                a = num;
                b = temp;
            }
            self.former.push(a);
            self.latter.push(std::cmp::Reverse(b));
            self.mid = None;
        } else {
            let left = *self.former.peek().unwrap();
            if num < left {
                self.former.push(num);
                self.mid = Some(self.former.pop().unwrap());
            } else {
                self.latter.push(std::cmp::Reverse(num));
                self.mid = Some(self.latter.pop().unwrap().0);
            }
        }
    }
    
    fn find_median(&self) -> f64 {
        if let Some(temp) = self.mid {
            return temp as f64;
        }
        let left = *self.former.peek().unwrap();
        let right = self.latter.peek().unwrap().0;
        return (left + right) as f64 / 2.0;
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
fn main() {
    let mut test = MedianFinder::new();
    test.add_num(12);
    test.add_num(10);
    test.add_num(13);
    test.test();
    test.add_num(11);
    test.test();
    test.add_num(5);
    test.test();
    let res = test.find_median();
    println!("{}", res);
}

#[cfg(test)]
mod hello {
    use super::*;
    #[test]
    fn aa() {
        let a = 22;
        let b = "hello this is a test";
        dbg!("hello mayday");
        dbg!(a);
        dbg!(b);
    }
}
