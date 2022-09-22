#[get("/add/<number>")]
pub fn add(number: u8) -> String {
    format!("{}", number + 1)
}
