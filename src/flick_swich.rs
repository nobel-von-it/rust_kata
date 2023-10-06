pub fn flick_switch(list: &[&str]) -> Vec<bool> {
    let mut v: Vec<bool> = vec![];
    let mut flag = true;
    for i in list.to_vec() {
        if i == "flick" {
            flag = !flag
        }
        v.push(flag)
    }
    v
}
pub fn flick_switch2(list: &[&str]) -> Vec<bool> {
    let mut f = true;
    list.iter().map(|&e| if e != "flick" {f} else {f = !f; f}).collect()
}