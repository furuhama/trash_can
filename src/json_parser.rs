use serde_json;
use serde_json::Value;

enum Media {
    Reddit,
    HackerNews,
}

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
        let v: Value = serde_json::from_str(&json).unwrap();
        let children = &v["data"]["children"];

        let mut json_vec = Vec::<Json>::new();

        for child in children.as_array().unwrap().iter() {
            let title = child["data"]["title"].as_str().unwrap().to_string();
            let url = child["data"]["permalink"].as_str().unwrap().to_string();

            json_vec.push(Json::new(title, url, Media::Reddit));
        }

        json_vec
    }

    pub fn get_url(&self) -> String {
        match self.media {
            Media::Reddit => format!("{}{}", String::from("https://www.reddit.com"), self.url),
            Media::HackerNews => { panic!("No Implement Error!"); String::from("hoge") }
        }
    }
}
