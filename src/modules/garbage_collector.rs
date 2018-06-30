use modules::{request_sender, json_parser, trash_can};
use std::time::Instant;

// GarbageCollector is an interface b/w outer world and inner this library
// GarbageCollector has TrashCan(server) and Trash(formatted contents),
// and connects them by using a reference
#[derive(Debug)]
pub struct GarbageCollector {
    trash: Vec<trash_can::Trash>,
    trash_can:trash_can::TrashCan,
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
        let trash = Vec::<trash_can::Trash>::new();
        let trash_can = trash_can::TrashCan::new();

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
            let res = request_sender::get_response(dotenv!("REDDIT_URI"));
            let jsons = json_parser::Json::parse_as_reddit(res);
            let trash = trash_can::Trash::new(String::from("Reddit best topics"), jsons);
            self.trash.push(trash);
        });


        measure!("HackerNews", {
            // process for hackernews
            let res = request_sender::get_response_hackernews((dotenv!("HACKERNEWS_URI").to_string() + "/topstories.json").as_str());
            let jsons = json_parser::Json::parse_as_hackernews(res);
            let trash =trash_can::Trash::new(String::from("HackerNews best topics"), jsons);
            self.trash.push(trash);
        });

        println!("============== Garbage collection finished ==============");
    }
}
