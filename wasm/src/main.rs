use crate::http::http_fetch;

mod raw;
mod http;

fn main() {
    http_fetch("https://swapi.dev/api/people/1");
    println!("Hello World");
}
