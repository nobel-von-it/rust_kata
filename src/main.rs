mod html_special_chars;
mod double_char;
mod set_alarm;
mod prs;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    println!("{:?}", 1);
    println!("{:?}", now.elapsed());
}
