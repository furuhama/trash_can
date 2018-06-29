use serde_json;
use serde_json::Value;

#[derive(Debug)]
enum Media {
    Reddit,
    HackerNews,
}

#[derive(Debug)]
pub struct Json {
    pub title: String,
    pub url: String,
    media: Media,
}

impl Json {
    fn new(title: String, url: String, media: Media) -> Json {
        Json {
            title: title,
            url: url,
            media: media,
        }
    }

    pub fn parse_as_reddit(json: String) -> Vec<Json> {
        let value: Value = serde_json::from_str(&json).unwrap();
        let children = &value["data"]["children"];

        let mut json_vec = Vec::<Json>::new();

        for child in children.as_array().unwrap().iter() {
            let title = child["data"]["title"].as_str().unwrap().to_string();
            let url = child["data"]["permalink"].as_str().unwrap().to_string();

            json_vec.push(Json::new(title, url, Media::Reddit));
        }

        json_vec
    }

    pub fn parse_as_hackernews(json: String) -> Vec<Json> {
        let value: Value = serde_json::from_str(&json).unwrap();

        let mut json_vec = Vec::<Json>::new();

        for child in value.as_array().unwrap().iter() {
            let title = child["title"].as_str().unwrap().to_string();
            let url = child["id"].as_u64().unwrap().to_string();

            println!("{} {}", title, url);

            json_vec.push(Json::new(title, url, Media::HackerNews));
        }

        json_vec
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_url(&self) -> String {
        match self.media {
            Media::Reddit => String::from("https://www.reddit.com") + self.url.as_str(),
            Media::HackerNews => String::from("https://news.ycombinator.com/item?id=") + self.url.as_str(),
        }
    }
}
