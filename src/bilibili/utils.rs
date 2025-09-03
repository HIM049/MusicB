use rand_agents::user_agent;
use regex::Regex;

pub fn get_user_agent() -> String {
    // "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36".to_string()
    user_agent()
}

pub fn extract_title(text: &str) -> Option<String> {
    let re = Regex::new(r"《([^》]+)》").unwrap();
    re.captures(text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}