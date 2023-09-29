use std::time::Instant;
mod first_non_consecutive;
mod find_difference;
mod combat;
mod switch_it_up;
mod hello;
mod two_sort;
mod bin_to_dec;
mod calc_bmi;
mod find_average;
mod power_of_two;
mod make_uppercase;
mod min_max;
mod count_pos_sum_neg;
mod sum_pos;

fn main() {
    let now = Instant::now();
    println!("{:?}", two_sort::two_sort2(&["bitcoin", "take", "over", "the"]));
    println!("{:?}", now.elapsed())
}
