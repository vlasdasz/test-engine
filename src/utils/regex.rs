
use regex::Regex;

pub fn find_match(str: &String, query: &str) -> String {
    let re = Regex::new(query).unwrap();
    let mat = re.find(&str).unwrap();
    String::from(mat.as_str())
}
