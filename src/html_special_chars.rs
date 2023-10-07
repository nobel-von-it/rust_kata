use std::time::Instant;

pub fn html_special_chars(html: &str) -> String {
    html.to_string()
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
}

#[test]
fn simple_test() {
    let now = Instant::now();
    assert_eq!(html_special_chars("<h2>Hello World</h2>"), "&lt;h2&gt;Hello World&lt;/h2&gt;");
    println!("{:?}", now.elapsed())
}