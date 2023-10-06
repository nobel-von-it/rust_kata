use std::time::Instant;
mod palindr;
mod two_sum;
mod flick_swich;
mod str_repeat;
mod bonus_time;
mod to_alternating_case;
mod str_to_array;
mod correct_tail;

fn main() {
    let now = Instant::now();
    println!("{:?}", correct_tail::correct_tail("fox", 'x'));
    println!("{:?}", now.elapsed())
}
