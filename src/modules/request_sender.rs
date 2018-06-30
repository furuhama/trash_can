use reqwest;
use serde_json;
use serde_json::Value;
extern crate dotenv;

pub fn get_response(uri: &str) -> String {
    let mut res = reqwest::get(uri).unwrap();
    res.text().unwrap()
}

// MEMO: I want to separate http request process & json parsing process
// However, HN api returns just ids of posts, so more http requests are needed for getting more
// info about each post(such as title, url, ...etc)
//
// TODO: separate their responsibilities beuatifully
pub fn get_response_hackernews(uri: &str) -> String {
    // first http request (just get ids of posts)
    let raw_json = get_response(uri);

    // parse first http response & send request to get more info
    let value: Value = serde_json::from_str(&raw_json).unwrap();
    let mut json_for_return = String::from("[");
    for idx in 0..20 {
        let id = value[idx].as_u64().unwrap().to_string();
        let uri_tmp = dotenv!("HACKERNEWS_URI").to_string() + "/item/" + &id + ".json";
        json_for_return += get_response(&uri_tmp).as_str();

        if idx != 19 {
            json_for_return += ",";
        }
    }

    json_for_return += "]";

    json_for_return
}
