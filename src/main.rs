extern crate genki;
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;

use genki::http_client;
use genki::json_parser;
use genki::http_server;

fn main() {
    let res = http_client::get_response(dotenv!("REDDIT_URI"));

    let jsons = json_parser::Json::parse_as_reddit(res);

    for json in jsons {
        println!("title: {}, url: {}", json.title, json.url);
    };

    http_server::wake_up();
}
