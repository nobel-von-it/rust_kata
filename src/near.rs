pub fn nearest_sq(n: u32) -> u32 {
    let mut nq = f64::sqrt(n as f64);
    nq = nq.round();
    (nq * nq) as u32
}

// or best practices

pub fn nearpp(n: u32) -> u32 {
    ((n as f64).sqrt().round() as u32).pow(2)
}