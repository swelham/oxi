extern crate glob;

pub mod finder;
mod template;

use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::fs::File;

pub fn parse(buf: PathBuf) -> Result<template::Template, io::Error> {
    let mut f = try!(File::open(buf));

    let mut contents = String::new();
    try!(f.read_to_string(&mut contents));

    Ok(template::Template {
        content: contents
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn should_return_template_instance() {
        let path = PathBuf::from("./tests/fixtures/bare.roxi");
        let template = super::parse(path).unwrap();
        let content = template.content;

        assert_eq!(content, "doctype html\nhtml\n    head\n    body\n".to_string());
    }
}
