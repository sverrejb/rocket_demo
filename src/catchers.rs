use rocket::Request;

#[catch(400)]
pub fn bad_request(req: &Request) -> String {
    //Stygg hack
    let error = req.local_cache(|| "");
    format!("400, bad request: {error}")
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Vi fant ingenting på '{}'. Prøv noe annet?", req.uri())
}
