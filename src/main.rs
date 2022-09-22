#[macro_use]
extern crate rocket;
use rocket_dyn_templates::Template;
mod catchers;
mod json;
mod math;
mod secrets;
mod templating;

#[get("/<name>/<age>")]
fn hello_age(name: &str, age: u8) -> String {
    format!("{}, du er {} år gammel!", name, age)
}

#[get("/<name>/<nickname>", rank = 2)]
fn hello_nick(name: &str, nickname: &str) -> String {
    format!("Hallo, {name} AKA {nickname}!")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", routes![hello_age, hello_nick])
    //.register("/", catchers![catchers::not_found, catchers::bad_request])
    //.mount("/math", routes![math::add])
    //.mount("/public", rocket::fs::FileServer::from("static/"))
    //.mount("/sensitive", routes![secrets::sensitive])
    //.mount("/json", routes![json::todo, json::new_todo])
    //.attach(Template::fairing())
    //.mount("/template", routes![templating::hello])
}
