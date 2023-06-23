use rouille::router;
use std::fs::{File, read_to_string};

mod utils;

fn main() {

    println!("Now listening to localhost:8080");
    rouille::start_server("localhost:8080", move |request| {
        router!(request,
                (GET) (/) => {
                    let file = File::open("files/index.html").unwrap();
                    rouille::Response::from_file("text/html", file)
                },

                (GET) (/panic) => {
                    rouille::Response::text("You have met with a terrible fate")
                },

                (GET) (/styles/{id: String}) => {
                    let css = File::open(format!("files/styles/{}", id)).unwrap();
                    rouille::Response::from_file("text/css", css)
                },

                (GET) (/scripts/{id: String}) => {
                    let file = File::open(format!("files/scripts/{}", id)).unwrap();
                    rouille::Response::from_file("text/css", file)
                },

                (GET) (/id/{id: String}) => {
                    let file = read_to_string(format!("files/notes/{}.md", id)).unwrap();
                    let file = utils::convert_to_html(file);
                    println!("{file}");

                    rouille::Response::from_data("text/html", file)
                },

                _ => {
                    let file_404 = File::open("files/404.html").unwrap();
                    rouille::Response {
                        status_code: 404,
                        headers: vec![],
                        data: rouille::ResponseBody::from_file(file_404),
                        upgrade: None,
                    }
                },
        )
    });

}
