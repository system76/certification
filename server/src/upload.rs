use std::io;

use file::File;

#[post("/upload", data="<file>")]
fn index(file: File) -> io::Result<String> {
    Ok(format!("{:?}", file))
}