use reqwest;
use std::io;

pub fn get_response() {
    let mut res = reqwest::get("https://www.reddit.com/.json?feed=762979e144d58e098da718c0f05b57faf1f7f835&user=Furuhama").unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}
