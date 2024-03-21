use std::time::Instant;

use rand::Rng;

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
    let mut v = vec![];
    let mut vres = vec![];
    for _i in 0..10 {
        v.push(rand::thread_rng().gen_range(0i32..100i32));
    }
    println!("{:?}", v);

    let now = Instant::now();
    for _i in 0..100 {
        let tmp: i32 = v.clone().iter().product();
        vres.push(tmp);
    }
    println!("{vres:?}");
    println!("{:?}", now.elapsed());
}
