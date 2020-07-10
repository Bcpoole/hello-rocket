#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}!", name.as_str())
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, hello])
        .mount("/public", StaticFiles::from("static"))
        .launch();
}
