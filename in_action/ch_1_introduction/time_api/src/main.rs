#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use chrono::prelude::*;
use rocket::response::content::Html;
use rocket_contrib::json::Json;

#[derive(Serialize)]
struct Timestamp {
    t: String,
}

#[get("/")]
fn index() -> Html<String> {
    let content: &str = "
    <h1>Hello, Rust in Action!</h1>
    <p> What is the <a href=\"/now\">time</a>?</p>
    ";
    let content_as_string = String::from(content);
    Html(content_as_string)
}

#[get("/now")]
fn now() -> Json<Timestamp> {
    let now: DateTime<Utc> = Utc::now();
    let timestamp = Timestamp { t: now.to_rfc3339() };
    Json(timestamp)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, now])
        .launch();
}