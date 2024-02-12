use std::path::Path;
use std::fs::read_to_string;
use crate::diagnostic::diagnostic;
use crate::utils::span::Span;

pub fn read_file(path: &String) -> Result<String, diagnostic::Diagnostic> {
    let absolute = Path::new(path);

    if !absolute.exists() {
        return Err(diagnostic::Diagnostic::new(
            diagnostic::Kind::FileNotFound,
            Span::new(String::from("cli"), 1, 1, 0, 0),
            format!("could not find file: {}", absolute.file_name().unwrap().to_str().unwrap())
        ));
    };

    if !absolute.is_file() {
        return Err(diagnostic::Diagnostic::new(
            diagnostic::Kind::FileNotRegular,
            Span::new(String::from("cli"), 1, 1, 0, 0),
            format!("file not regular: {}", absolute.file_name().unwrap().to_str().unwrap())
        ));
    };

    match read_to_string(absolute) {
        Ok(content) => Ok(content),
        Err(_) => Err(diagnostic::Diagnostic::new(
            diagnostic::Kind::FileNotReadable,
            Span::new(String::from("cli"), 1, 1, 0, 0),
            format!("file not readable: {}", absolute.file_name().unwrap().to_str().unwrap())
        ))
    }
}
