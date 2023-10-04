use std::time::Instant;
mod palindr;
mod two_sum;

fn main() {
    let now = Instant::now();
    println!("{:?}", palindr::is_palindrome(123));
    println!("{:?}", now.elapsed())
}
