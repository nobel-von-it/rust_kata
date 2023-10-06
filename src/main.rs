use std::time::Instant;
mod palindr;
mod two_sum;
mod flick_swich;
mod str_repeat;
mod bonus_time;

fn main() {
    let now = Instant::now();
    println!("{:?}", bonus_time::bonus_time(1000, true));
    println!("{:?}", now.elapsed())
}
