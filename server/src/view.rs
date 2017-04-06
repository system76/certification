use rocket_contrib::Template;
use std::io;
use test::Test;
use util;

#[get("/view/<model>")]
fn model(model: &str) -> io::Result<Template> {
    #[derive(Serialize)]
    struct Context {
        version: &'static str,
        model: String,
        tests: Vec<String>,
    }

    Ok(Template::render("view/model", &Context {
        version: util::version(),
        model: model.to_string(),
        tests: util::list_tests(model)?
    }))
}

#[get("/view/<model>/<test>")]
fn test(model: &str, test: &str) -> io::Result<Template> {
    #[derive(Serialize)]
    struct Context {
        version: &'static str,
        model: String,
        test: String,
        data: Test,
    }

    Ok(Template::render("view/test", &Context {
        version: util::version(),
        model: model.to_string(),
        test: test.to_string(),
        data: Test::from_str(&util::read_test(model, test)?)?
    }))
}