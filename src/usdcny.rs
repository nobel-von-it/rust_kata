pub fn usdcny(usd: u16) -> String {
    format!("{:.2} Chinese Yuan", 6.75*usd as f32)
}