use multipart::server::save::SavedFile;
use std::{fs, path, io};
use std::io::Read;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn create_model(model: &str) -> io::Result<String> {
    let model = sanitize(model)?;

    fs::create_dir(format!("tests/{}", model))?;
    Ok(model)
}

pub fn create_test(model: &str, file: &TempFile) -> io::Result<(String, String)> {
    let model = sanitize(model)?;

    let test = if let Some(ref filename) = file.0.filename {
        sanitize(&filename)?
    } else {
        return Err(io::Error::new(io::ErrorKind::Other, "Filename was empty"));
    };
    file.copy(&format!("tests/{}/{}", model, test))?;
    Ok((model, test))
}

pub fn list_models() -> io::Result<Vec<String>> {
    list("tests", |entry| {
        if let Ok(file_type) = entry.file_type() {
            file_type.is_dir()
        } else {
            false
        }
    })
}

pub fn list_tests(model: &str) -> io::Result<Vec<String>> {
    let model = sanitize(model)?;

    list(&format!("tests/{}", model), |entry| {
        if let Ok(file_type) = entry.file_type() {
            file_type.is_file()
        } else {
            false
        }
    })
}

pub fn read_test(model: &str, test: &str) -> io::Result<String> {
    let model = sanitize(model)?;
    let test = sanitize(test)?;

    let mut file = fs::File::open(&format!("tests/{}/{}", model, test))?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

fn list<F: Fn(&fs::DirEntry) -> bool>(path: &str, filter: F) -> io::Result<Vec<String>> {
    let mut entries = Vec::new();

    for entry_res in fs::read_dir(path)? {
        let entry = entry_res?;
        if filter(&entry) {
            if let Ok(file_name) = entry.file_name().into_string() {
                entries.push(file_name);
            }
        }
    }

    entries.sort();

    Ok(entries)
}

fn sanitize(part: &str) -> io::Result<String> {
    let sanitized: String = part.chars().filter(|&c| c != '/').collect();
    if ! sanitized.is_empty() {
        Ok(sanitized)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Sanitized path was empty"))
    }
}

#[derive(Debug)]
pub struct TempFile(pub SavedFile);

impl TempFile {
    fn copy<P: AsRef<path::Path>>(&self, path: P) -> io::Result<u64> {
        fs::copy(&self.0.path, path)
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        println!("Removing {}", self.0.path.display());
        if let Err(e) = fs::remove_file(&self.0.path) {
            println!("ERROR: Could not remove temp file {:?}: {:?}", self.0, e);
        }
    }
}