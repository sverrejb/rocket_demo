use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Task {
    name: String,
    done: bool,
}

#[get("/todo")]
pub fn todo() -> Json<Task> {
    Json(Task {
        name: "Hold talk på backend summit".to_string(),
        done: false,
    })
}

#[post("/todo", data = "<task>")]
pub fn new_todo(task: Json<Task>) -> Json<Task> {
    //Do something interesting
    print!("{:?}", task);
    task
}
