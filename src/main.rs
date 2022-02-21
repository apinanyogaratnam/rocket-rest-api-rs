#![feature(proc_macro_hygiene, decl_macro)]
use rocket::*;
use rocket_contrib::json;

#[get("/")]
fn root() -> json::JsonValue {
    json!({"message": "Hello, world!"})
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    rocket::ignite().mount("/", routes![root, hello]).launch();
}
