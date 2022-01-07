#[macro_use] extern crate rocket;


#[get("/user/<id>")]
fn user(id: usize) { /* ... */ }

#[get("/user/<id>", rank = 2)]
fn user_int(id: isize) { /* ... */ }

#[get("/user/<id>", rank = 3)]
fn user_str(id: &str) { /* ... */ }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![user, user_int, user_str])
}