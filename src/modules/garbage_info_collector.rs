use modules::{json_parser, request_sender, trash_can};
use std::time::Instant;

// GarbageInfoCollector is an interface b/w outer world and inner this library
// GarbageInfoCollector has TrashCan(server) and Trash(formatted contents),
// and connects them by using a reference
#[derive(Debug)]
pub struct GarbageInfoCollector {
    trash: Vec<trash_can::Trash>,
    trash_can: trash_can::TrashCan,
}

impl GarbageInfoCollector {
    pub fn init() -> Self {
        println!("Initializing GarbageInfoCollector...");

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
            trash,
            trash_can,
        }
    }

    fn init_contents(&mut self) {
        println!("============== Garbage Info collection started ===============");

        // define macro to measure time
        macro_rules! measure {
            ($msg:expr, $x:expr) => {{
                let start = Instant::now();
                let expr_result = $x;
                let end = start.elapsed();

                println!(
                    "It took {}.{:03} sec to collect garbage info from {}",
                    end.as_secs(),
                    end.subsec_millis(),
                    $msg
                );

                expr_result
            }};
        }

        measure!("Reddit", {
            // process for reddit
            let res = request_sender::get_response(dotenv!("REDDIT_URI"));
            let jsons = json_parser::Json::parse_as_reddit(&res);
            let trash = trash_can::Trash::new(String::from("Reddit best topics"), jsons);
            self.trash.push(trash);
        });

        measure!("HackerNews", {
            // process for hackernews
            let res = request_sender::get_response_hackernews(
                (dotenv!("HACKERNEWS_URI").to_string() + "/topstories.json").as_str(),
            );
            let jsons = json_parser::Json::parse_as_hackernews(&res);
            let trash = trash_can::Trash::new(String::from("HackerNews best topics"), jsons);
            self.trash.push(trash);
        });

        println!("============== Garbage Info collection finished ==============");

        measure!("Re: HackerNews", {
            request_sender::get_hackernews((dotenv!("HACKERNEWS_URI").to_string() + "/topstories.json").as_str());
        });
    }
}
