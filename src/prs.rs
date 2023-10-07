use std::time::Instant;

fn rps(p1: &str, p2: &str) -> &'static str {
    let results = ["Draw!", "Player 1 won!", "Player 2 won!"];
    if p1 == p2 {
        return results[0]
    }
    return if p1 == "scissors" && p2 == "paper" {
        results[1]
    } else if p1 == "paper" && p2 == "rock" {
        results[1]
    } else if p1 == "rock" && p2 == "scissors" {
        results[1]
    } else {
        results[2]
    };
}
// or
fn rps2(p1: &str, p2: &str) -> &'static str {
    if p1 == p2 { return "Draw!" }
    match (p1, p2) {
        ("scissors", "paper") | ("paper", "rock") | ("rock", "scissors") => "Player 1 won!",
        _ => "Player 2 won!"
    }
}

#[test]
fn simple_test() {
    let now = Instant::now();
    assert_eq!(rps("scissors", "paper"), "Player 1 won!");
    assert_eq!(rps("paper", "scissors"), "Player 2 won!");
    assert_eq!(rps("rock", "rock"), "Draw!");
    println!("{:?}", now.elapsed())
}

#[test]
fn simp_test() {
    let now = Instant::now();
    assert_eq!(rps2("scissors", "paper"), "Player 1 won!");
    assert_eq!(rps2("paper", "scissors"), "Player 2 won!");
    assert_eq!(rps2("rock", "rock"), "Draw!");
    println!("{:?}", now.elapsed())
}