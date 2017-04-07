use rocket_contrib::Template;
use std::{cmp, io};
use test::Test;
use util;

#[get("/view/<model>")]
fn model(model: &str) -> io::Result<Template> {
    #[derive(Serialize)]
    struct TestRow {
        name: String,
        failed: usize,
        passed: usize,
        not_supported: usize,
        total: usize,
    }

    #[derive(Serialize)]
    struct Context {
        version: &'static str,
        model: String,
        tests: Vec<TestRow>,
    }

    let mut tests = Vec::new();
    for test in util::list_tests(model)? {
        let data = Test::from_str(&util::read_test(model, &test)?)?;

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

        tests.push(TestRow {
            name: test,
            failed: failed,
            passed: passed,
            not_supported: not_supported,
            total: total,
        });
    }

    Ok(Template::render("view/model", &Context {
        version: util::version(),
        model: model.to_string(),
        tests: tests
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

    let mut data = Test::from_str(&util::read_test(model, test)?)?;
    if let Some(ref mut results) = data.results {
        results.sort_by(|a, b| match a.status.cmp(&b.status) {
            cmp::Ordering::Equal => a.id.cmp(&b.id),
            not_equal => not_equal
        });
    }

    Ok(Template::render("view/test", &Context {
        version: util::version(),
        model: model.to_string(),
        test: test.to_string(),
        data: data
    }))
}