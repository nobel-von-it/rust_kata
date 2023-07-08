pub fn feast(beast: &str, dish: &str) -> bool {
    let mut be = beast.chars();
    let mut di = dish.chars();

    if &di.nth(0).unwrap() == &be.nth(0).unwrap()
        && &di.nth_back(0).unwrap() == &be.nth_back(0).unwrap() {
        true
    } else { false }
}
// or best practices

pub fn feastpp(beast: &str, dish: &str) -> bool {
    beast.chars().next() == dish.chars().next() &&
        beast.chars().last() == dish.chars().last()
}