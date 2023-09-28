pub fn two_sort(arr: &[&str]) -> String {
    let mut min: &str = "zzzzzzzzzzzzzzzz";
    for i in arr {
        if min > i {
            min = i
        }
    }
    let mut res = String::new();
    for i in min.chars() {
        res.push(i);
        if min.find(i).unwrap() == min.len() - 1 {
            break
        }
        res.push_str("***")
    }
    res
}
pub fn two_sort2(arr: &[&str]) -> String {
    let mut res = arr.iter().min().expect("is_empty").to_string();
    for i in (1..res.len()).rev() {
        res.insert_str(i, "***")
    }
    res
}