pub fn bmi(weight: u32, height: f32) -> &'static str {
    let bmi = weight as f32 / (height * height);
    let mut res = "";
    if bmi <= 18.5 {
        res = "Underweight"
    } else if bmi <= 25.0 {
        res = "Normal"
    } else if bmi <= 30.0 {
        res = "Overweight"
    } else {
        res = "Obese"
    }
    res
}