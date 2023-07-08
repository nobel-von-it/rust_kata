pub fn get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;

    for i in string.chars() {
        if "aeiou".find(i).is_some() {
            vowels_count += 1;
        }
    }
    vowels_count
}
