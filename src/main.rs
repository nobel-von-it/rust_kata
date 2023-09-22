use std::time::Instant;
use crate::quarter_of::quarter_of;

mod quarter_of;
mod warn_the_sheep;
mod other_angle;
mod usdcny;
mod contamination;

fn main() {
    let now = Instant::now();
    println!("{:?}", contamination::contamination("sdf", "a"));
    println!("{:?}", now.elapsed())
}
