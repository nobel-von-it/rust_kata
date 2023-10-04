pub fn neutralise(s1: &str, s2: &str) -> String {
    assert_eq!(s1.len(), s2.len());
    let v1: Vec<&str> = s1.split("").collect();
    let v2: Vec<&str> = s2.split("").collect();
    let mut str = String::new();
    for i in 0..s1.len()+1 {
        if v1[i] != v2[i] {
            str.push_str("0")
        } else {
            str.push_str(v1[i])
        }
    }
    str
}