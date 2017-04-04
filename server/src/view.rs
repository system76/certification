use rocket_contrib::Template;

#[get("/view/<model>")]
fn model(model: &str) -> Template {
    #[derive(Serialize)]
    struct Context {
        version: &'static str,
        model: String,
        tests: Vec<String>,
    }

    Template::render("view/model", &Context {
        version: env!("CARGO_PKG_VERSION"),
        model: model.to_string(),
        tests: vec!["test_1".to_string()]
    })
}

#[get("/view/<model>/<test>")]
fn test(model: &str, test: &str) -> Template {
    #[derive(Serialize)]
    struct Context {
        version: &'static str,
        model: String,
        test: String,
    }

    Template::render("view/test", &Context {
        version: env!("CARGO_PKG_VERSION"),
        model: model.to_string(),
        test: test.to_string(),
    })
}