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
#[must_use]
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
#[must_use]
pub fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let arr2 = arr.to_vec();
    let mut res_arr = vec![];
    let mut zeros = 0;
    for i in arr2 {
        if i != 0 {
            res_arr.push(i);
        } else {
            zeros += 1;
        }
    }
    for _i in 0..zeros {
        res_arr.push(0);
    }
    res_arr
}
#[must_use]
pub fn move_zeros_smart(arr: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(arr.len());
    res.extend(arr.iter().filter(|&&x| x != 0));
    res.resize(arr.len(), 0);
    res
}
