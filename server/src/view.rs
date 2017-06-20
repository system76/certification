use rocket_contrib::Template;
use std::{cmp, io};
use test::Test;
use util;

#[get("/")]
fn index() -> io::Result<Template> {
    #[derive(Serialize)]
    struct ModelRow {
        model: String,
        test_count: usize,
        test: String,
        name: String,
        date: String,
        time: String,
        failed: usize,
        not_supported: usize,
        passed: usize,
        skipped: usize,
        total: usize,
    }

    #[derive(Serialize)]
    struct Context {
        models: Vec<ModelRow>,
    }

    let mut models = Vec::new();
    for model in util::list_models()? {
        let mut tests = util::list_tests(&model)?;
        let test_count = tests.len();
        if let Some(test) = tests.pop() {
            let data = Test::from_str(&util::read_test(&model, &test)?)?;

            let mut failed = 0;
            let mut not_supported = 0;
            let mut passed = 0;
            let mut skipped = 0;
            let mut total = 0;
            if let Some(results) = data.results {
                for result in results {
                    match result.status.as_str() {
                        "failed" => failed += 1,
                        "passed" => passed += 1,
                        "not supported" => not_supported += 1,
                        "skipped" => skipped += 1,
                        status => println!("Unknown status {}", status)
                    }
                    total += 1;
                }
            }

            let name;
            let date;
            let time;
            {
                let mut parts = test.split('_');
                date = parts.next().unwrap_or("").to_string();
                time = parts.next().unwrap_or("").to_string();
                name = parts.next().unwrap_or("").to_string();
            }

            models.push(ModelRow {
                model: model,
                test_count: test_count,
                test: test,
                name: name,
                date: date,
                time: time,
                failed: failed,
                not_supported: not_supported,
                passed: passed,
                skipped: skipped,
                total: total,
            });
        }
    }

    Ok(Template::render("index", &Context {
        models: models
    }))
}

#[get("/<model>")]
fn model(model: &str) -> io::Result<Template> {
    #[derive(Serialize)]
    struct TestRow {
        test: String,
        name: String,
        date: String,
        time: String,
        failed: usize,
        not_supported: usize,
        passed: usize,
        skipped: usize,
        total: usize,
    }

    #[derive(Serialize)]
    struct Context {
        model: String,
        tests: Vec<TestRow>,
    }

    let mut tests = Vec::new();
    for test in util::list_tests(model)? {
        let data = Test::from_str(&util::read_test(model, &test)?)?;

        let mut failed = 0;
        let mut not_supported = 0;
        let mut passed = 0;
        let mut skipped = 0;
        let mut total = 0;
        if let Some(results) = data.results {
            for result in results {
                match result.status.as_str() {
                    "failed" => failed += 1,
                    "passed" => passed += 1,
                    "not supported" => not_supported += 1,
                    "skipped" => skipped += 1,
                    status => println!("Unknown status {}", status)
                }
                total += 1;
            }
        }

        let name;
        let date;
        let time;
        {
            let mut parts = test.split('_');
            date = parts.next().unwrap_or("").to_string();
            time = parts.next().unwrap_or("").to_string();
            name = parts.next().unwrap_or("").to_string();
        }

        tests.push(TestRow {
            test: test,
            name: name,
            date: date,
            time: time,
            failed: failed,
            not_supported: not_supported,
            passed: passed,
            skipped: skipped,
            total: total,
        });
    }

    Ok(Template::render("model", &Context {
        model: model.to_string(),
        tests: tests
    }))
}

#[get("/<model>/<test>")]
fn test(model: &str, test: &str) -> io::Result<Template> {
    #[derive(Serialize)]
    struct Context {
        model: String,
        test: String,
        name: String,
        date: String,
        time: String,
        failed: usize,
        not_supported: usize,
        passed: usize,
        skipped: usize,
        total: usize,
        data: Test,
    }

    let mut failed = 0;
    let mut not_supported = 0;
    let mut passed = 0;
    let mut skipped = 0;
    let mut total = 0;
    let mut data = Test::from_str(&util::read_test(model, test)?)?;
    if let Some(ref mut results) = data.results {
        for result in results.iter() {
            match result.status.as_str() {
                "failed" => failed += 1,
                "passed" => passed += 1,
                "not supported" => not_supported += 1,
                "skipped" => skipped += 1,
                status => println!("Unknown status {}", status)
            }
            total += 1;
        }

        results.sort_by(|a, b| match a.status.cmp(&b.status) {
            cmp::Ordering::Equal => a.id.cmp(&b.id),
            not_equal => not_equal
        });
    }

    let name;
    let date;
    let time;
    {
        let mut parts = test.split('_');
        date = parts.next().unwrap_or("").to_string();
        time = parts.next().unwrap_or("").to_string();
        name = parts.next().unwrap_or("").to_string();
    }

    Ok(Template::render("test", &Context {
        model: model.to_string(),
        test: test.to_string(),
        name: name,
        date: date,
        time: time,
        failed: failed,
        not_supported: not_supported,
        passed: passed,
        skipped: skipped,
        total: total,
        data: data
    }))
}
