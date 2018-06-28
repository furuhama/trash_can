use nickel::Nickel;

pub fn wake_up() {
    let mut server = Nickel::new();
    server.utilize(router! {
        get "**" => |_req, _res| {
            "Hello world!"
        }
    });

    server.listen("127.0.0.1:3000");
}
