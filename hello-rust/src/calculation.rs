// Following will be automatically imported
// extern crate std;
// use std::prelude::v1::*;

pub fn calculation() {
    pub fn answer() -> () {
        let a = 40;
        let b = 2;
        assert_eq!(sum(a, b), 42);
    }

    pub fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    answer();
}