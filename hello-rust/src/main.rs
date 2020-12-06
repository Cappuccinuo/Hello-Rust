use ferris_says::say;
use std::io::{stdout, BufWriter};
mod calculation;
mod fizz_buzz;
use fizz_buzz::fizz_buzz;
// Ref: https://www.rust-lang.org/learn/get-started

// Had to run following during `cargo build`
// rm -rf ~/.cargo/registry/index/*
// rm -rf ~/.cargo/.package-cache
// https://stackoverflow.com/questions/47565203/cargo-build-hangs-with-blocking-waiting-for-file-lock-on-the-registry-index-a?rq=1
// to fix Blocking waiting for file lock on package cache
fn main() {
    print_new_line();
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans");
    let width = message.chars().count();
    
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    calculation::calculation();

    print_new_line();
    let x = &temp();
    println!("{}", x);
    // invalid left-hand side expression
    // temp() = *x;

    print_new_line();
    let a = 1;
    // Cannot assign twice to immutable variable a
    // a = 2;
    let mut b = 2;
    println!("b = {}", b); // 2
    b = 3;
    println!("a = {}", a); // 1
    println!("b = {}", b); // 3

    // Ownership
    print_new_line();
    let place1 = "hello";
    let place2 = "hello".to_string();
    let other = place1;
    println!("{:?}", other); // "hello"
    let other = place2;
    println!("{:?}", other); // Should throw err that other value used here after move, but not, get "hello"

    print_new_line();
    let a = [1, 2, 3];
    let b = &a;
    println!("{:p}", b); // 0x7ffeefbfebc4
    println!("{:?}", *b); // [1, 2, 3]
    let mut c = vec![1, 2, 3];
    let d = &mut c;
    d.push(4);
    println!("{:?}", d); // [1, 2, 3, 4]
    let e = &42;
    assert_eq!(42, *e);

    // fizz_buzz
    assert_eq!(fizz_buzz(15), "fizzbuzz".to_string());
    assert_eq!(fizz_buzz(3), "fizz".to_string());
    assert_eq!(fizz_buzz(5), "buzz".to_string());
    assert_eq!(fizz_buzz(13), "13".to_string());
}

pub fn temp() -> i32 {
    return 1;
}

pub fn print_new_line() -> () {
    println!("=================");
}