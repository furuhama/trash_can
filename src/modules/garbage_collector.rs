use modules::{http_client, json_parser, http_server};
use std::time::Instant;

// TrashCan(server) と Trash(contents) の実体を持ち、二つを参照で繋げる役目
#[derive(Debug)]
pub struct GarbageCollector {
    trash: Vec<http_server::Trash>,
    trash_can: http_server::TrashCan,
}

impl GarbageCollector {
    pub fn init() -> Self {
        println!("Initializing GarbageCollector...");

        let mut gc = Self::new();

        gc.init_contents();

        gc
    }

    pub fn work(&mut self) {
        println!("Preparing Trash Can...");
        self.trash_can.wake_up(&self.trash);
    }

    fn new() -> Self {
        let trash = Vec::<http_server::Trash>::new();
        let trash_can = http_server::TrashCan::new();

        Self {
            trash: trash,
            trash_can: trash_can,
        }
    }

    fn init_contents(&mut self) {
        println!("============== Garbage collection started ===============");

        // define macro to measure time
        macro_rules! measure {
            ($msg:expr, $x:expr) => {
                {
                    let start = Instant::now();
                    let expr_result = $x;
                    let end = start.elapsed();

                    println!("It took {}.{:03} sec to collect garbage from {}", end.as_secs(), end.subsec_nanos() / 1_000_000, $msg);

                    expr_result
                }
            };
        }

        measure!("Reddit", {
            // process for reddit
            let res = http_client::get_response(dotenv!("REDDIT_URI"));
            let jsons = json_parser::Json::parse_as_reddit(res);
            let trash = http_server::Trash::new(String::from("Reddit best topics"), jsons);
            self.trash.push(trash);
        });


        measure!("HackerNews", {
            // process for hackernews
            let res = http_client::get_response_hackernews((dotenv!("HACKERNEWS_URI").to_string() + "/topstories.json").as_str());
            let jsons = json_parser::Json::parse_as_hackernews(res);
            let trash = http_server::Trash::new(String::from("HackerNews best topics"), jsons);
            self.trash.push(trash);
        });

        println!("============== Garbage collection finished ==============");
    }
}
