use itertools::Either;

pub fn sum_mix(a: &[Either<i32, String>]) -> i32 {
    let mut r = 0;
    for i in a {
        match i {
            Either::Left(x) => {r += x}
            Either::Right(x) => {r += x.parse::<i32>().unwrap()}
        }
    }
    r
}