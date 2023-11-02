use std::time::Instant;
use crate::kata;

#[test]
fn html_test() {
    let now = Instant::now();
    assert_eq!(kata::kyu8::html_special_chars("<h2>Hello World</h2>"), "&lt;h2&gt;Hello World&lt;/h2&gt;");
    println!("{:?}", now.elapsed())
}
#[test]
fn rps_test() {
    let now = Instant::now();
    assert_eq!(kata::kyu8::rps1("scissors", "paper"), "Player 1 won!");
    assert_eq!(kata::kyu8::rps1("paper", "scissors"), "Player 2 won!");
    assert_eq!(kata::kyu8::rps1("rock", "rock"), "Draw!");
    println!("{:?}", now.elapsed());
    assert_eq!(kata::kyu8::rps2("scissors", "paper"), "Player 1 won!");
    assert_eq!(kata::kyu8::rps2("paper", "scissors"), "Player 2 won!");
    assert_eq!(kata::kyu8::rps2("rock", "rock"), "Draw!");
    println!("{:?}", now.elapsed())
}