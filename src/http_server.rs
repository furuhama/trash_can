use nickel::{Nickel, HttpRouter};
use json_parser::Json;

pub struct Server<'a> {
    jsons: &'a Vec<Json>,
}

impl<'a> Server<'a> {
    pub fn new(jsons: &'a Vec<Json>) -> Self {
        Self {
            jsons: &jsons,
        }
    }

    pub fn wake_up(&self) {
        let mut server = Nickel::new();
        let html = Self::generate_html(self.jsons);

        server.get("/", middleware! { |_, _res|
            html.as_str()
        });

        server.listen("127.0.0.1:3000").unwrap();
    }

    fn generate_html(jsons: &Vec<Json>) -> String {
        let mut html = String::from("<html><head><title>Genki -dairy info platform-</title><meta charset=\"utf-8\"></head><body><h1>Reddit best topics</h1><ul>");

        for json in jsons {
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
