use std::fmt::{Display, write} ;

struct MyWrap(Vec<i32>) ;

impl Display for MyWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

fn main() {

    let v = vec![1, 2, 3, 4, 5] ;
    // печать без :? невозможна
    println!("{:?}", v) ;   // Out: [1, 2, 3, 4, 5]

    let my_wrap = MyWrap(vec![1, 2, 3, 4, 5]) ;

    println!("{}", my_wrap) ;   // Out: [1, 2, 3, 4, 5]
}
