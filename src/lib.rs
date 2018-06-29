extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate nickel;
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;

pub mod http_client;
pub mod json_parser;
pub mod http_server;
