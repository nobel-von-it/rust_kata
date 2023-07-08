use crate::disemvowel::disemvowel;
use crate::feast::feast;
use crate::mult35::solution;
use crate::near::nearest_sq;
use crate::sumarr::sum_of_differences;
use crate::table::{mt2, multi_table};
use crate::vowel::get_count;

mod sumarr;
mod feast;
mod near;
mod table;
mod mult35;
mod evenodd;
mod vowel;
mod disemvowel;

fn main() {
    println!("{}",disemvowel("lol"))
}
