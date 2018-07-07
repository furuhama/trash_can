use reqwest;
use serde_json;
use serde_json::Value;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;
use std::time::Instant;
use futures::Future;
use futures::future::join_all;
use hyper::{Uri, Client};
use hyper::client::HttpConnector;
use tokio_core::reactor::Core;

// define macro to measure time
macro_rules! measure {
    ($x:expr) => {{
        let start = Instant::now();
        let expr_result = $x;
        let end = start.elapsed();

        println!(
            "It took {}.{:03} sec to send request concurrently",
            end.as_secs(),
            end.subsec_millis()
        );

        expr_result
    }};
}

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
    let max = 20;
    // first http request (just get ids of posts)
    let raw_json = get_response(uri);

    // parse first http response & send request to get more info
    let value: Value = serde_json::from_str(&raw_json).unwrap();
    let uri_head = dotenv!("HACKERNEWS_URI").to_string() + "/item/";

    // Try to do proecss concurrently.
    // However it didn't get faster than it was a single thread.
    // To send request(the line contains -> `get_response(&uri_tmp)`) may not be done concurrently
    // in my expectation.
    // TODO: try to solve this and make this function faster
    let target = Arc::new(Mutex::new(vec![String::from(""); 20]));
    let (tx, rx) = mpsc::channel();

    measure!({
        for idx in 0..max {
            let (value, target, tx, uri_head) = (value.clone(), target.clone(), tx.clone(), uri_head.clone());

            let id = value[idx].as_u64().unwrap().to_string();

            thread::spawn(move || {
                let mut target = target.lock().unwrap();
                let uri_tmp = uri_head + &id + ".json";
                target[idx] = get_response(&uri_tmp).as_str().to_string();

                tx.send(()).unwrap();
            });
        }

        for _ in 0..max {
            rx.recv().unwrap();
        }
    });

    let mut json_for_return = String::from("[");

    let target = match (*target).lock() {
        Ok(v) => v,
        _ => panic!("hoge"),
    };

    for i in 0..max {
        json_for_return += target[i].as_str();
        if i != max - 1 {
            json_for_return += " ,";
        }
    }

    json_for_return += "]";

    json_for_return
}

fn http_request_job(client: &Client<HttpConnector>, index: usize, url: String) -> Box<Future<Item=(), Error=()>> {
    let uri: Uri = url.parse().unwrap();

    let job = client.get(uri).then(move |result| -> Result<(), ()> {
        match result {
            Ok(response) => {
                println!("{:2}: {} => {}",index, url, response.status());
            },
            Err(error) => {
                let _ = println!("{:2}: Error: {}", index, error);
            }
        }
        Ok(())
    });
    Box::new(job)
}

pub fn get_hackernews(url: &str) {
    let get_url = |u: &str| -> String { dotenv!("HACKERNEWS_URI").to_string() + "/item/" + u };

    let raw_json = get_response(url);
    let mut value: Value = serde_json::from_str(&raw_json).unwrap();
    println!("get urls");
    let urls = value.as_array_mut().unwrap().into_iter().map(|v| get_url(&v.as_u64().unwrap().to_string()));


    let mut core = Core::new().unwrap();
    let client = Client::new();

    let jobs = urls.into_iter().enumerate()
        .map(|(i, url)| http_request_job(&client, i + 1, url));

    core.run(join_all(jobs)).unwrap();
}
