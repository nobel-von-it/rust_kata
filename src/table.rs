use itertools::Itertools;

pub fn multi_table(n: u64) -> String {
    let mut res = String::new();
    for i in 1..11 {
        res += (&*(i.to_string()
            + " * " + &*n.to_string()
            + " = " + &*(i as u64 * n).to_string()));
        if i != 10 {
            res += "\n"
        }

    }
    res
}

//or best practices

pub fn mt2(n: u64) -> String {
    (1..=10).map(|a| format!("{} * {} = {}", a, n, a * n)).join("\n")
}

