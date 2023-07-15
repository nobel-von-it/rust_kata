pub fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
    let mut res: Vec<u32> = cubes.to_vec();
    match dir {
        'R' => res.sort(),
        'L' => {res.sort(); res.reverse()}
        _ => {}
    }
    res
}

// or minimize

pub fn flipp(dir: char, cubes: &[u32]) -> Vec<u32> {
    let mut res = cubes.to_vec();
    res.sort();
    if dir == 'L' {res.reverse()}
    res
}