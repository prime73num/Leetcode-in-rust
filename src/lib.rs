

pub struct Tt {
    
}

#[cfg(test)]
mod hello {
    use super::*;
    #[test]
    fn aa() {
        let a = 222;
        let b = "hello";
        dbg!(a);
        dbg!(b);
    }
}
