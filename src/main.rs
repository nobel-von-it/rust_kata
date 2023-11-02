mod html_special_chars;
mod double_char;
mod set_alarm;
mod prs;

use std::time::Instant;
pub mod kata_rs;
fn main() {
    let now = Instant::now();

    println!("{:?}", 1);
    println!("{:?}", now.elapsed());
}
