use rocket::*;

#[post("/run", data = "<functions>")]
fn run(functions: String) -> String {
    match nast::run(functions.as_str()) {
        Ok(v) => format!("{v:?}"),
        Err(e) => format!("Error: {e}"),
    }
}

#[post("/info", data = "<functions>")]
fn info(functions: String) -> String {
    match nast::get_info(functions.as_str()) {
        Ok(result) => result,
        Err(e) => format!("Error: {e}"),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![run, info])
}
