use ferris_says::say;
use std::io::{stdout, BufWriter};
mod calculation;
// Ref: https://www.rust-lang.org/learn/get-started

// Had to run following during `cargo build`
// rm -rf ~/.cargo/registry/index/*
// rm -rf ~/.cargo/.package-cache
// https://stackoverflow.com/questions/47565203/cargo-build-hangs-with-blocking-waiting-for-file-lock-on-the-registry-index-a?rq=1
// to fix Blocking waiting for file lock on package cache
fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans");
    let width = message.chars().count();
    
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    calculation::calculation();

    let x = &temp();
    println!("{}", x);
    // invalid left-hand side expression
    // temp() = *x;

    let a = 1;
    // Cannot assign twice to immutable variable a
    // a = 2;
    let mut b = 2;
    println!("b = {}", b); // 2
    b = 3;
    println!("a = {}", a); // 1
    println!("b = {}", b); // 3
}

pub fn temp() -> i32 {
    return 1;
}