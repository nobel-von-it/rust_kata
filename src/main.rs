use std::time::Instant;
use crate::quarter_of::quarter_of;

mod quarter_of;
mod warn_the_sheep;

fn main() {
    let now = Instant::now();
    println!("{:?}",quarter_of(11));
    println!("{:?}", now.elapsed())
}
