use std::time::Instant;

pub mod kata;
pub mod maybe_kata;

// in the kata module:
// ---
// kata is a ready-made code, divided by complexity,
// there are also some tests,
// but I will not write tests for all codes;
// ---
// maybe_kata is where I will write any code before structuring and optimizing.
// ---
fn main() {
    let now = Instant::now();
    println!("{:?}", 1);
    println!("{:?}", now.elapsed());
}
