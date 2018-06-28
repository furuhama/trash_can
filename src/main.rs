extern crate genki;
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;

use genki::http_client;

fn main() {
    http_client::get_response(dotenv!("REDDIT_URI"));
}
