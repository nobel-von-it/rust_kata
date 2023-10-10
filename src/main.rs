use std::time::Instant;
mod palindr;
mod two_sum;
mod flick_swich;
mod str_repeat;
mod bonus_time;
mod to_alternating_case;
mod cannon_ready;

fn main() {
    let now = Instant::now();
    println!("{:?}", to_alternating_case::to_alternating_case("heLLo"));
    println!("{:?}", now.elapsed())
}
