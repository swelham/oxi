use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::fs::File;

pub fn transpile(buf: PathBuf) -> Result<String, io::Error> {
    let template = try!(Template::from(buf));
    Ok(template.content)

    // TODO: parse template
    // TODO: render template
}

pub struct Template {
    pub content: String
}

impl Template {
    fn from(buf: PathBuf) -> Result<Template, io::Error> {
        let mut f = try!(File::open(buf));

        let mut contents = String::new();
        try!(f.read_to_string(&mut contents));

        Ok(Template{
            content: contents
        })
    }
}
