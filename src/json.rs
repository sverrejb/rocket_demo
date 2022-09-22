use rocket::response::status;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Task {
    description: String,
    done: bool,
    note: Option<String>,
}

#[get("/todo")]
pub fn todo() -> Json<Task> {
    Json(Task {
        description: "Hold talk på backend summit".to_string(),
        done: false,
        note: None,
    })
}

#[post("/todo", data = "<task>")]
pub fn new_todo(task: Json<Task>) -> status::Accepted<Json<Task>> {
    //Do something interesting
    print!("{:?}", task);
    status::Accepted(Some(task))
}
