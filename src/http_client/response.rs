use reqwest;
use std::io;

pub fn get_response(uri: &str) {
    let mut res = reqwest::get(uri).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}
