
fn warn_the_sheep(queue: &[&str]) -> String {
    let i = queue.iter().position(|&r| r == "wolf").unwrap();

    if i == queue.len() - 1 {
        return format!("Pls go away and stop eating my sheep")
    }
    format!("Oi! Sheep number {}! You are about to be eaten by a wolf!", queue.len() - i - 1)
}
// or
fn warn_the_sheep_best(q: &[&str]) -> String {
    match q.iter().rev().position(|&e| e == "wolf").unwrap() {
        0 => format!("Pls go away and stop eating my sheep"),
        n => format!("Oi! Sheep number {}! You are about to be eaten by a wolf!", n)
    }
}

#[test]
fn test() {
    assert_eq!(
        warn_the_sheep(&["sheep", "sheep", "sheep", "sheep", "sheep", "wolf", "sheep", "sheep"]),
        "Oi! Sheep number 2! You are about to be eaten by a wolf!",
    );
    assert_eq!(
        warn_the_sheep(&["sheep", "wolf", "sheep", "sheep", "sheep", "sheep", "sheep"]),
        "Oi! Sheep number 5! You are about to be eaten by a wolf!",
    );
    assert_eq!(
        warn_the_sheep(&["wolf", "sheep", "sheep", "sheep", "sheep", "sheep", "sheep"]),
        "Oi! Sheep number 6! You are about to be eaten by a wolf!",
    );
    assert_eq!(
        warn_the_sheep(&["sheep", "wolf", "sheep"]),
        "Oi! Sheep number 1! You are about to be eaten by a wolf!",
    );
    assert_eq!(
        warn_the_sheep(&["sheep", "sheep", "wolf"]),
        "Pls go away and stop eating my sheep",
    );
}