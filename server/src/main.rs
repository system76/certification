#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate multipart;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::Template;

mod file;
mod upload;
mod view;

pub fn models() -> Vec<&'static str> {
    vec![
        "galp2",
        "gaze12",
        "kudu4",
    ]
}

#[get("/")]
fn index() -> Template {
    #[derive(Serialize)]
    struct Context {
        version: &'static str,
        models: Vec<&'static str>,
    }

    Template::render("index", &Context {
        version: env!("CARGO_PKG_VERSION"),
        models: models()
    })
}

fn main() {
    rocket::ignite().mount("/", routes![
        index,
        upload::index,
        view::model,
        view::test
    ]).launch();
}