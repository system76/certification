use multipart::server::Multipart;
use multipart::server::save::SaveResult;
use rocket::data::{self, Data, FromData};
use rocket::request::Request;
use rocket::response::Redirect;
use rocket::outcome::Outcome::*;
use rocket::http::Status;
use std::io;

use util::{self, TempFile};

#[derive(Debug)]
pub struct Upload {
    model: String,
    file: TempFile
}

impl FromData for Upload {
    type Error = ();

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, ()> {
        if req.content_type().is_none() {
            return Forward(data);
        }

        let content_type = req.content_type().unwrap();
        if ! content_type.is_data_form() {
            println!("WARN: Form data does not have multipart content type.");
            return Forward(data);
        }

        println!("Found a multipart request!");
        let mut params = content_type.params();
        let boundary = match params.next() {
            Some(("boundary", value)) => value,
            _ => {
                println!("WARN: Content-Type did not contain multipart boundary.");
                return Forward(data);
            }
        };

        let mut model_opt: Option<String> = None;
        let mut file_opt: Option<TempFile> = None;

        println!("Found multipart with boundary value: {}", boundary);
        let mut multipart = Multipart::with_body(data.open(), boundary);
        loop {
            match multipart.read_entry() {
                Ok(Some(mut field)) => match field.name.as_str() {
                    "model" => if let Some(text) = field.data.as_text() {
                        model_opt = Some(text.to_string());
                    } else {
                        println!("Invalid data for model");
                        return Failure((Status::UnprocessableEntity, ()));
                    },
                    "file" => if let Some(file) = field.data.as_file() {
                        if file.filename.is_some() {
                            println!("Found a file ({:?}) with name: {:?}", file.content_type, file.filename);
                            match file.save().size_limit(4 * 1024 * 1024).temp() {
                                SaveResult::Full(saved_file) => {
                                    println!("Saved the file somewhere: {:?}", saved_file);
                                    file_opt = Some(TempFile(saved_file));
                                },
                                SaveResult::Partial(temp_file, reason) => {
                                    println!("ERROR: Partially saved to temp file {:?}: {:?}", temp_file, reason);
                                    drop(TempFile(temp_file));
                                    return Failure((Status::InternalServerError, ()));
                                },
                                SaveResult::Error(e) => {
                                    println!("ERROR: Could not save to temp file: {:?}", e);
                                    return Failure((Status::InternalServerError, ()));
                                }
                            }
                        } else {
                            println!("Empty filename for file");
                            return Failure((Status::UnprocessableEntity, ()));
                        }
                    } else {
                        println!("Invalid data for file");
                        return Failure((Status::UnprocessableEntity, ()));
                    },
                    _ => {
                        println!("Unknown field: {}", field.name);
                        return Failure((Status::UnprocessableEntity, ()));
                    }
                },
                Ok(None) => {
                    break;
                }
                Err(e) => {
                    println!("ERROR: Failed to read multipart entries: {:?}", e);
                    return Failure((Status::InternalServerError, ()));
                }
            }
        }

        let model = match model_opt {
            Some(model) => model,
            None => {
                println!("Did not find model");
                return Failure((Status::UnprocessableEntity, ()));
            }
        };

        let file = match file_opt {
            Some(file) => file,
            None => {
                println!("Did not find file");
                return Failure((Status::UnprocessableEntity, ()));
            }
        };

        Success(Upload {
            model: model,
            file: file
        })
    }
}

#[post("/upload", data="<upload>")]
fn index(upload: Upload) -> io::Result<Redirect> {
    let (model, test) = util::create_test(&upload.model, &upload.file)?;
    Ok(Redirect::to(&format!("/view/{}/{}", model, test)))
}
