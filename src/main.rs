use std::time::Instant;
mod first_non_consecutive;
mod find_difference;

fn main() {
    let now = Instant::now();
    println!("{:?}", find_difference::find_difference(&[3,2,5], &[1,4,4]));
    println!("{:?}", now.elapsed())
}
