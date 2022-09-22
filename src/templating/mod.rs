use rocket_dyn_templates::{context, Template};

#[get("/hello/<name>")]
pub fn hello(name: &str) -> Template {
    Template::render(
        "index",
        context! {
            title: "Hello",
            name: Some(name),
            items: vec!["One", "Two", "Three"],
        },
    )
}
