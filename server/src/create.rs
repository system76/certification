use rocket::request::Form;
use rocket::response::Redirect;
use std::io;
use util;

#[derive(FromForm)]
struct CreateForm {
    model: String
}

#[post("/create", data="<form>")]
fn index(form: Form<CreateForm>) -> io::Result<Redirect> {
    let model = util::create_model(&form.get().model)?;
    Ok(Redirect::to(&format!("/view/{}", model)))
}