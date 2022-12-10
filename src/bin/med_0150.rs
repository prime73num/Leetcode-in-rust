













// 150. Evaluate Reverse Polish Notation
// Stack and HHeap 6



struct Solution {}
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut res: Vec<i32> = Vec::new();
        for i in tokens {
            match i.as_str() {
                "+" => {
                    let b = res.pop().unwrap();
                    let a = res.pop().unwrap();
                    res.push(a+b);
                },
                "-" => {
                    let b = res.pop().unwrap();
                    let a = res.pop().unwrap();
                    res.push(a-b);
                },
                "*" => {
                    let b = res.pop().unwrap();
                    let a = res.pop().unwrap();
                    res.push(a*b);
                },
                "/" => {
                    let b = res.pop().unwrap();
                    let a = res.pop().unwrap();
                    res.push(a/b);
                },
                _ => {
                    println!("{}", i);
                    res.push(i.parse::<i32>().unwrap());
                }
            }
        }
        res.pop().unwrap()
    }
}
fn main() {
}
