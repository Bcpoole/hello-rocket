#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use rocket::http::{Cookie, Cookies, RawStr};
use rocket::request::{Form, FromFormValue, LenientForm};
use rocket::response::{Flash, Redirect};
use rocket::Data;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::str;

#[derive(Deserialize, Debug, FromForm)]
struct User {
    name: String,
    account: usize,
}

#[get("/")]
fn index(cookies: Cookies) -> String {
    cookies
        .get("message")
        .map(|value| format!("Message: {}", value))
        .unwrap_or_else(|| "Hello, world!".to_string())
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

#[get("/user/<id>", format = "json")]
fn user(mut cookies: Cookies, id: usize) -> String {
    cookies.add_private(Cookie::new("user_id", id.to_string()));
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

/// Retrieve the user's ID, if any.
#[get("/user_id")]
fn user_id(mut cookies: Cookies) -> Option<String> {
    cookies
        .get_private("user_id")
        .map(|cookie| format!("User ID: {}", cookie.value()))
}

//example http://localhost:8000/item?id=123&name=Bob&account=307
#[get("/item?<id>&<user..>")]
fn item(id: usize, user: Option<Form<User>>) -> String {
    user.map(|u| format!("item {} for user {} aka {}!", id, u.name, u.account))
        .unwrap_or_else(|| format!("item {}", id))
}

#[post("/user", format = "json", data = "<user>")]
fn new_user(user: Json<User>) -> String {
    format!("new user {} aka {}!", user.name, user.account)
}

#[derive(FromForm)]
struct Task {
    description: String,
    #[form(field = "type")]
    api_type: String,
}

#[post("/todo", data = "<task>")]
fn new_task(task: Form<Task>) -> String {
    format!("{} |{}", task.api_type, task.description)
}

#[post("/todol", data = "<task>")]
fn new_task_lenient(task: LenientForm<Task>) -> String {
    format!("Trimmed to: {}", task.description)
}

struct AdultAge(usize);

impl<'v> FromFormValue<'v> for AdultAge {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<AdultAge, &'v RawStr> {
        match form_value.parse::<usize>() {
            Ok(age) if age >= 21 => Ok(AdultAge(age)),
            _ => Err(form_value),
        }
    }
}

#[derive(FromForm)]
struct Person {
    age: AdultAge,
}

const LIMIT: u64 = 256;

#[post("/upload", format = "plain", data = "<data>")]
fn upload(data: Data) -> Result<String, std::io::Error> {
    data.stream_to_file("tmp/upload.txt").map(|n| n.to_string())
}

// Upload but limit size
#[post("/upload_limit", format = "plain", data = "<data>")]
fn upload_limit(data: Data) -> Result<String, std::io::Error> {
    let mut buffer = [0; LIMIT as usize];
    data.open().take(LIMIT).read(&mut buffer).unwrap();
    let msg = str::from_utf8(&buffer)
        .unwrap()
        .trim_matches(char::from(0))
        .to_string();
    let mut file = File::create("tmp/upload.txt")?;
    file.write_all(msg.as_bytes())?;
    Ok(msg)
}

/// Remove the `user_id` cookie.
#[post("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                hello,
                hello_wave,
                user,
                user_int,
                user_str,
                user_id,
                new_user,
                item,
                new_task,
                new_task_lenient,
                upload,
                upload_limit,
                logout
            ],
        )
        .mount("/public", StaticFiles::from("static"))
}

fn main() {
    rocket().launch();
}
