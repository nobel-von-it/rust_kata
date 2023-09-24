use std::time::Instant;
use crate::quarter_of::quarter_of;

mod quarter_of;
mod warn_the_sheep;
mod other_angle;
mod usdcny;
mod contamination;
mod find_mutl;
mod invert;
mod merge_arrays;
mod sum_mixed_array;
mod fake_bit;
mod abbr;

fn main() {
    let now = Instant::now();
    println!("{:?}", fake_bit::fake_bin("3489592376491716397323956896"));
    println!("{:?}", now.elapsed())
}
