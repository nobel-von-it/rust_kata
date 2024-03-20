pub fn beeramid(bonus: i32, price: f32) -> usize {
    let mut bank: i32 = (bonus as f32 / price) as i32;
    if bank <= 0 {
        return 0;
    }

    let mut count = 0usize;
    while bank > 0 {
        count += 1;
        bank -= count.pow(2) as i32;
    }
    if bank < 0 {
        count - 1
    } else {
        count
    }
}
pub fn beeramid_one_line(bonus: i32, price: f32) -> usize {
    let count = (bonus as f32 / price) as i32;
    (1i32..)
        .scan(0, |acc, x| {
            *acc += x.pow(2);
            Some(*acc)
        })
        .take_while(|&x| x <= count)
        .count()
}
