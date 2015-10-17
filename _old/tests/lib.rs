extern crate oxi;

use std::fs::File;
use std::io::prelude::*;
use oxi::compiler::Compiler;

#[test]
fn html_basic() {
    // TODO: tidy up all of the strings
    let expected = read_file("./tests/expectations/html_basic.html");
    let expected_pretty = read_file("./tests/expectations/html_basic_pretty.html");

    let output = match Compiler::compile("./tests/fixtures/html_basic.oxit", false) {
        Ok(out) => out.to_string(),
        Err(e) => {
            return assert!(false, format!("Compiler::compile: {}", e));
        }
    };

    let output_pretty = match Compiler::compile("./tests/fixtures/html_basic.oxit", true) {
        Ok(out) => out.to_string(),
        Err(e) => {
            return assert!(false, format!("Compiler::compile: {}", e));
        }
    };

    assert_eq!(output, expected);
    assert_eq!(output_pretty, expected_pretty);
}

#[test]
fn xml_basic() {
    // TODO: tidy up all of the strings
    let expected = read_file("./tests/expectations/xml_basic.xml");
    let expected_pretty = read_file("./tests/expectations/xml_basic_pretty.xml");

    let output = match Compiler::compile("./tests/fixtures/xml_basic.oxit", false) {
        Ok(out) => out.to_string(),
        Err(e) => {
            return assert!(false, format!("Compiler::compile: {}", e));
        }
    };

    let output_pretty = match Compiler::compile("./tests/fixtures/xml_basic.oxit", true) {
        Ok(out) => out.to_string(),
        Err(e) => {
            return assert!(false, format!("Compiler::compile: {}", e));
        }
    };

    assert_eq!(output, expected);
    assert_eq!(output_pretty, expected_pretty);
}

fn read_file(path: &str) -> String {
    let mut f = File::open(path).unwrap();

    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    return contents;
}
