#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate multipart;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use rocket_contrib::Template;
use std::{fs, io};

mod create;
mod test;
mod upload;
mod util;
mod view;

#[get("/")]
fn index() -> io::Result<Template> {
    #[derive(Serialize)]
    struct Context {
        version: &'static str,
        models: Vec<String>,
    }

    Ok(Template::render("index", &Context {
        version: util::version(),
        models: util::list_models()?
    }))
}

fn main() {
    // Create test dir if it does not exist
    fs::create_dir_all("tests").unwrap();

    rocket::ignite().mount("/", routes![
        index,
        create::index,
        upload::index,
        view::model,
        view::test
    ]).launch();
}