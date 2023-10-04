pub fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    let res = cap - on - wait;
    if res > 0 {
        0
    } else {
        -res
    }
}

pub fn enough2(c: i32, o: i32, w: i32) -> i32 {
    (o + w - c).max(0)
}