#[macro_use]
extern crate rocket;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/user")]
fn get_users() -> &'static str {
    "user1, user2, user_n...!"
}

#[post("/user")]
fn post_users() -> &'static str {
    "user1"
}

#[put("/user")]
fn put_users() -> &'static str {
    "user1_update"
}

#[delete("/user")]
fn delete_users() -> &'static str {
    "DATA DELETED"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hello", routes![world])
        .mount("/", routes![get_users, post_users, put_users, delete_users])
}
