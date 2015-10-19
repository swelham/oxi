extern crate glob;

pub mod finder;
mod template;
mod parser;

use std::path::PathBuf;

pub fn transpile(buf: PathBuf) -> String {
    let content = match template::transpile(buf) {
        Ok(out) => out,
        Err(e) => panic!(e)
    };

    content
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    // #[test]
    // fn should_return_template_instance() {
    //     let path = PathBuf::from("./tests/fixtures/bare.roxi");
    //     let content = super::transpile(path);
    //
    //     assert_eq!(content, "<!DOCTYPE html><html><head></head><body></body></html>".to_string());
    // }
}
