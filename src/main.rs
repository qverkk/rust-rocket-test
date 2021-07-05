#![feature(proc_macro_hygiene, decl_macro)]

use rocket::serde::{json::Json, Deserialize};

use rocket::{get, launch, post, routes};

#[derive(Deserialize)]
struct Person<'r> {
    name: &'r str,
    last_name: &'r str,
}

#[get("/")]
fn index() -> &'static str {
    "some test"
}

#[get("/test")]
fn test() -> &'static str {
    "test"
}

#[post("/testpost", data = "<person>")]
fn testpost(person: Json<Person<'_>>) -> &'_ str {
    person.name
}

// fn main() {
//     rocket::ignite()
//         .mount("/", routes![index, test, testpost])
//         .launch();
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, test, testpost])
}
