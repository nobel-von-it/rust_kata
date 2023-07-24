pub fn quarter_of(month: u8) -> u8 {
    (month as f32 / 3.0).ceil() as u8
}

#[test]
fn test() {
    assert_eq!(quarter_of(11), 4);
    assert_eq!(quarter_of(1), 1);
}