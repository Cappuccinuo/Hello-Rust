use ferris_says::say;
use std::io::{stdout, BufWriter};
mod calculation;
mod functions;
use functions::*;
// Ref: https://www.rust-lang.org/learn/get-started

// Had to run following during `cargo build`
// rm -rf ~/.cargo/registry/index/*
// rm -rf ~/.cargo/.package-cache
// https://stackoverflow.com/questions/47565203/cargo-build-hangs-with-blocking-waiting-for-file-lock-on-the-registry-index-a?rq=1
// to fix Blocking waiting for file lock on package cache
fn main() {
    print_hello_fellow_rustaceans();
    verify_expression();
    verify_invlaid_value_expression();
    verify_mutable_and_immutable();
    verify_ownership();
    verify_fizz_buzz();
    verify_lexical_scope();
    verify_function_pointer();
    verify_const_function();
}

fn temp() -> i32 {
    return 1;
}

fn print_new_line() -> () {
    println!("=================");
}

fn print_hello_fellow_rustaceans() -> () {
    print_new_line();
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans");
    let width = message.chars().count();
    
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

fn verify_expression() -> () {
    calculation::calculation();
}

fn verify_invlaid_value_expression() -> () {
    // 2-3
    print_new_line();
    let x = &temp();
    println!("{}", x);
    // invalid left-hand side expression
    // temp() = *x;
}

fn verify_mutable_and_immutable() -> () {
    // 2-4
    print_new_line();
    let a = 1;
    // Cannot assign twice to immutable variable a
    // a = 2;
    let mut b = 2;
    println!("b = {}", b); // 2
    b = 3;
    println!("a = {}", a); // 1
    println!("b = {}", b); // 3
}

fn verify_fizz_buzz() -> () {
    // 2-8
    assert_eq!(fizz_buzz(15), "fizzbuzz".to_string());
    assert_eq!(fizz_buzz(3), "fizz".to_string());
    assert_eq!(fizz_buzz(5), "buzz".to_string());
    assert_eq!(fizz_buzz(13), "13".to_string());
}

fn verify_ownership() -> () {
    // 2-5
    print_new_line();
    let place1 = "hello";
    let place2 = "hello".to_string();
    let other = place1;
    println!("{:?}", other); // "hello"
    let other = place2;
    println!("{:?}", other); // Should throw err that other value used here after move, but not, get "hello"

    // 2-6
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
}

fn verify_lexical_scope() -> () {
    // 2-9
    let v = "Hello World";
    assert_eq!(v, "Hello World");

    let v = "Hello Rust";
    assert_eq!(v, "Hello Rust");

    {
        let v = "Hello World";
        assert_eq!(v, "Hello World");
    }

    assert_eq!(v, "Hello Rust");
}

fn verify_function_pointer() -> () {
    // 2-10
    let a = 2;
    let b = 3;
    assert_eq!(math(sum, a, b), 5);
    assert_eq!(math(product, a, b), 6);

    // 2-11
    fn is_true() -> bool { true }
    fn true_maker() -> fn() -> bool { is_true }
    assert_eq!(true_maker()(), true);
}

fn verify_const_function() -> () {
    const fn init_len() -> usize {
        return 5;
    }
    let arr = [0; init_len()];
    for num in &arr {
        println!("{}", num); // an array inited with five zero
    }
}