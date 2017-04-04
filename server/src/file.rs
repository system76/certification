use rocket::data::{self, Data, FromData};
use rocket::request::Request;
use rocket::outcome::Outcome::*;
use rocket::http::Status;

use multipart::server::{Multipart, MultipartData, ReadEntryResult};
use multipart::server::save::{SavedFile, SaveResult};

#[derive(Debug)]
pub struct File(SavedFile);

impl FromData for File {
    type Error = ();

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, ()> {
        if req.content_type().is_none() {
            return Forward(data);
        }

        let content_type = req.content_type().unwrap();
        if !content_type.is_data_form() {
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

        println!("Found multipart with boundary value: {}", boundary);
        let multipart = Multipart::with_body(data.open(), boundary);
        let field = match multipart.into_entry() {
            ReadEntryResult::Entry(field) => field,
            ReadEntryResult::End(_) => {
                println!("ERROR: Form contained no fields.");
                return Failure((Status::UnprocessableEntity, ()));
            }
            ReadEntryResult::Error(_, e) => {
                println!("ERROR: The multipart form was malformed: {:?}.", e);
                return Failure((Status::BadRequest, ()));
            }
        };

        println!("Found field: {:?}", field.name);
        match field.data {
            MultipartData::Text(_text) => {
                println!("Did not find a file");
                return Failure((Status::UnprocessableEntity, ()));
            },
            MultipartData::File(mut file) => {
                println!("Found a file ({:?}) with name: {:?}", file.content_type, file.filename);

                match file.save().size_limit(4 * 1024 * 1024).temp() {
                    SaveResult::Full(temp_file) => {
                        println!("Saved the file somewhere: {:?}", temp_file);

                        return Success(File(temp_file));
                    },
                    SaveResult::Partial(temp_file, reason) => {
                        println!("ERROR: Partially saved to temp file {:?}: {:?}", temp_file, reason);
                        return Failure((Status::InternalServerError, ()));
                    },
                    SaveResult::Error(e) => {
                        println!("ERROR: Could not save to temp file: {:?}", e);
                        return Failure((Status::InternalServerError, ()));
                    }
                }
            }
        }
    }
}