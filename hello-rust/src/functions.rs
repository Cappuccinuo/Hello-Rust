pub fn fizz_buzz(num: i32) -> String {
    if num % 15 == 0 {
        // return "fizzbuzz"; // expected struct `std::string::String`, get `&str`
        return "fizzbuzz".to_string();
    } else if num % 3 == 0 {
        return "fizz".to_string();
    } else if num % 5 == 0 {
        return "buzz".to_string();
    } else {
        return num.to_string();
    }
}
