#![allow(unused)]
pub fn do_stuff() {
    println!("baz baz baz");
}

pub fn do_more_stuff() {
    println!("bazzzzzzzz");
}

pub mod submodule1 {
    pub fn func1() {}
    fn func2() {}

    pub mod subsubmodule2 {
        use super::*;
        pub fn func3() {
            func2();
            println!("D'oh");
        }
    }
}
