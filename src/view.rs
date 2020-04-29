use serde_json::Value;

pub fn display(pulls: &Vec<Value>) {
    for p in pulls.iter() {
        if let (Some(number), Some(url), Some(title)) = (p["number"].as_i64(), p["html_url"].as_str(), p["title"].as_str()) {
            println!("- [#{}]({}) {}", number, url, title);
        }
    }
}
