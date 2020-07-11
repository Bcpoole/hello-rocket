#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket::http::RawStr;
use rocket_contrib::serve::StaticFiles;

use rocket::request::Form;

#[derive(FromForm)]
struct User {
    name: String,
    account: usize,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}

#[get("/hello?wave&<name>")]
fn hello_wave(name: Option<String>) -> String {
    name.map(|name| format!("Hi, {}!", name))
        .unwrap_or_else(|| "Hello!".into())
}

#[get("/user/<id>")]
fn user(id: usize) -> String {
    format!("Hello (1) user {}!", id)
}

#[get("/user/<id>", rank = 2)]
fn user_int(id: isize) -> String {
    format!("Hello (2) user {}!", id)
}

#[get("/user/<id>", rank = 3)]
fn user_str(id: &RawStr) -> String {
    format!("Hello (3) user {}!", id.as_str())
}

//example http://localhost:8000/item?id=123&name=Bob&account=307
#[get("/item?<id>&<user..>")]
fn item(id: usize, user: Option<Form<User>>) -> String {
    user.map(|u| format!("item {} for user {} aka {}!", id, u.name, u.account))
        .unwrap_or_else(|| format!("item {}", id))
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![index, hello, hello_wave, user, user_int, user_str, item],
        )
        .mount("/public", StaticFiles::from("static"))
        .launch();
}
