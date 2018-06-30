extern crate trash_can;
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;

use trash_can::modules::{http_client, json_parser, http_server};

fn main() {
    // process for reddit
    let res_r = http_client::get_response(dotenv!("REDDIT_URI"));
    let jsons_r = json_parser::Json::parse_as_reddit(res_r);
    let content_r = http_server::Content::new(String::from("Reddit best topics"), &jsons_r);

    // process for hackernews
    let res_h = http_client::get_response_hackernews((dotenv!("HACKERNEWS_URI").to_string() + "/topstories.json").as_str());
    let jsons_h = json_parser::Json::parse_as_hackernews(res_h);
    let content_h = http_server::Content::new(String::from("HackerNews best topics"), &jsons_h);

    // init & wake up server
    let mut server = http_server::Server::new();
    server.add_content(content_r);
    server.add_content(content_h);
    server.wake_up();
}
