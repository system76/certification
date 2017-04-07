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
use test::Test;

mod test;
mod upload;
mod util;
mod view;

#[get("/")]
fn index() -> io::Result<Template> {
    #[derive(Serialize)]
    struct ModelRow {
        name: String,
        test: String,
        failed: usize,
        passed: usize,
        not_supported: usize,
        total: usize,
    }

    #[derive(Serialize)]
    struct Context {
        version: &'static str,
        models: Vec<ModelRow>,
    }

    let mut models = Vec::new();
    for model in util::list_models()? {
        if let Some(test) = util::list_tests(&model)?.pop() {
            let data = Test::from_str(&util::read_test(&model, &test)?)?;

            let mut failed = 0;
            let mut passed = 0;
            let mut not_supported = 0;
            let mut total = 0;
            if let Some(results) = data.results {
                for result in results {
                    match result.status.as_str() {
                        "failed" => failed += 1,
                        "passed" => passed += 1,
                        _ => not_supported += 1
                    }
                    total += 1;
                }
            }

            models.push(ModelRow {
                name: model,
                test: test,
                failed: failed,
                passed: passed,
                not_supported: not_supported,
                total: total,
            });
        }
    }

    Ok(Template::render("index", &Context {
        version: util::version(),
        models: models
    }))
}

fn main() {
    // Create test dir if it does not exist
    fs::create_dir_all("tests").unwrap();

    rocket::ignite().mount("/", routes![
        index,
        upload::index,
        view::model,
        view::test
    ]).launch();
}