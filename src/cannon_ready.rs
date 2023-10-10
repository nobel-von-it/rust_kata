use std::collections::HashMap;

fn cannons_ready(gunners: HashMap<&str, &str>) -> String {
    for (i, k) in gunners {
        if k == "nay" {
            return "Shiver me timbers!".to_string()
        }
    }
    "Fire!".to_string()
}