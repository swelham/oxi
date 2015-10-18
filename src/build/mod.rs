extern crate glob;

use std::path::PathBuf;

pub struct TemplateFinder {
    root_dir: &'static str
}

impl TemplateFinder {
    pub fn find_all(self) -> Vec<PathBuf> {
        // TODO: glob seems to have issues when a wildcard is
        //       preseeded by a windows seperator. adding a
        //       UNIX seperator (even if the string ends \\)
        //       still works. need to investigate!
        let mut path = String::from(self.root_dir);

        if !path.ends_with("/") {
            path.push_str("/");
        }

        path.push_str("**/*.roxi");

        let paths = glob::glob(path.as_ref()).unwrap();
        let mut bufs: Vec<PathBuf> = Vec::new();

        for path in paths.filter_map(Result::ok) {
            bufs.push(path);
        }

        bufs
    }
}


#[test]
fn should_find_templates() {
    let finder = TemplateFinder{
        root_dir: "./tests/fixtures/finder/"
    };
    let templates = finder.find_all();

    assert_eq!(3, templates.len());
}
