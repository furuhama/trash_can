use nickel::{Nickel, HttpRouter};
use json_parser::Json;

pub struct Server<'a> {
    contents: Vec<Content<'a>>,
}

impl<'a> Server<'a> {
    pub fn new() -> Self {
        Self {
            contents: Vec::<Content>::new(),
        }
    }

    pub fn wake_up(&self) {
        let mut server = Nickel::new();
        let html = self.generate_html();

        server.get("/", middleware! { |_, _res|
            html.as_str()
        });

        server.listen("127.0.0.1:3000").unwrap();
    }

    pub fn add_content(&mut self, content: Content<'a>) {
        self.contents.push(content);
    }

    fn generate_html(&self) -> String {
        let mut html = String::from("<html><head><title>Genki -dairy info platform-</title><meta charset=\"utf-8\"></head><body>");

        for content in &self.contents {
            html += &content.generate_html();
        }

        html
    }
}

pub struct Content<'a> {
    title_messsage: String,
    posts: &'a Vec<Json>,
}

impl<'a> Content<'a> {
    pub fn new(title_messsage: String, posts: &'a Vec<Json>) -> Self {
        Self {
            title_messsage: title_messsage,
            posts: posts,
        }
    }

    pub fn generate_html(&self) -> String {
        let mut html = String::from("<h1>") + &self.title_messsage + "</h1><ul>";

        for json in self.posts {
            html += "<li><a href=\"";
            html += &json.get_url();
            html += "\" target=\"_blank\">";
            html += &json.get_title();
            html += "</a></li>";
        };
        html += "</ul>";

        html
    }
}
