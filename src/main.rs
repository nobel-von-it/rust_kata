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

fn main() {
    let now = Instant::now();
    println!("{:?}", merge_arrays::merge_arrs(&[1,3,5,7,9,11,12], &[1,2,3,4,5,10,12]));
    println!("{:?}", now.elapsed())
}
