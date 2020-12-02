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
}
