#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate multipart;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use rocket_contrib::Template;
use std::fs;

mod test;
mod upload;
mod util;
mod view;

fn main() {
    // Create test dir if it does not exist
    fs::create_dir_all("tests").unwrap();

    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![
            upload::index,
            view::index,
            view::model,
            view::test
        ])
        .launch();
}
