
pub fn create_phone_number(numbers: &[u8]) -> String {
    let res = String::from(numbers);

    "(" + &res[..3] + ")" + " " + &res[3..6] + "-" + &res[6..]
}