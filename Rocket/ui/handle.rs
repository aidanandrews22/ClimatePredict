#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::Request;
use rocket::response::content;

#[get("/")]
fn index() -> content::Html<&'static str> {
    content::Html(include_str!("index.html"))
}

#[post("/predict", data = "<form_data>")]
fn predict(form_data: rocket::request::Form<FormInput>) -> String {
    let behavior = &form_data.behavior;
    let frequency = &form_data.frequency;
    let duration = &form_data.duration;

    // Here you can process the form data and return the result
    let result = format!(
        "Predicting the climate impact of {}, with a frequency of {} and a duration of {}.",
        behavior, frequency, duration
    );

    result
}

#[derive(FromForm)]
struct FormInput {
    behavior: String,
    frequency: String,
    duration: String,
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("404: Page '{}' not found", req.uri())
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, predict])
        .register(catchers![not_found])
        .launch();
}
