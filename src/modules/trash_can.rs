use modules::json_parser::Json;
use nickel::{HttpRouter, Nickel};

#[derive(Debug)]
pub struct TrashCan {}

impl TrashCan {
    pub fn new() -> Self {
        Self {}
    }

    pub fn wake_up(&self, trash: &Vec<Trash>) {
        let mut server = Nickel::new();
        let html = self.generate_html(trash);

        server.get(
            "/",
            middleware! { |_, _res|
                html.as_str()
            },
        );

        server.listen("127.0.0.1:3000").unwrap();
    }

    fn generate_html(&self, trash: &Vec<Trash>) -> String {
        let mut html = String::from("<html><head><title>Trash Can</title><meta charset=\"utf-8\"><style> * { box-sizing: border-box; } .column { float: left; width: 50%; padding: 10px;} .row:after { content: \"\"; display: table; clear: both;}</style></head><body><div class=\"row\">");

        for content in trash {
            html += &content.generate_html();
        }

        html += "</div></body></html>";

        html
    }
}

#[derive(Debug)]
pub struct Trash {
    title_messsage: String,
    posts: Vec<Json>,
}

impl Trash {
    pub fn new(title_messsage: String, posts: Vec<Json>) -> Self {
        Self {
            title_messsage: title_messsage,
            posts: posts,
        }
    }

    pub fn generate_html(&self) -> String {
        let mut html =
            String::from("<div class=\"column\"><h1>") + &self.title_messsage + "</h1><ul>";

        for json in &self.posts {
            html += "<li><a href=\"";
            html += &json.get_url();
            html += "\" target=\"_blank\">";
            html += &json.get_title();
            html += "</a></li>";
        }
        html += "</ul></div>";

        html
    }
}
