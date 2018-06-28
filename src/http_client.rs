use reqwest;

pub fn get_response(uri: &str) -> String {
    let mut res = reqwest::get(uri).unwrap();
    res.text().unwrap()
}
