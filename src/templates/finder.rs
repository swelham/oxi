extern crate glob;

use std::path::PathBuf;

pub struct TemplateFinder {
    pub root_dir: &'static str
}

impl TemplateFinder {
    pub fn find_all(self) -> Vec<PathBuf> {
        // TODO: glob seems to have issues when a wildcard is
        //       preseeded by a windows seperator. adding a
        //       UNIX seperator (even if the string ends \\)
        //       still works. need to investigate!
        let mut path = String::from(self.root_dir);

        if !path.ends_with(".roxi") {
            if !path.ends_with("/") {
                path.push_str("/");
            }

            path.push_str("**/*.roxi");
        }

        let paths = glob::glob(path.as_ref()).unwrap();
        let mut bufs: Vec<PathBuf> = Vec::new();

        for path in paths.filter_map(Result::ok) {
            bufs.push(path);
        }

        bufs
    }
}


#[cfg(test)]
mod tests {
    use super::TemplateFinder;

    #[test]
    fn should_find_single_template() {
        let finder = TemplateFinder{
            root_dir: "./tests/fixtures/empty.roxi"
        };
        let templates = finder.find_all();

        assert_eq!(1, templates.len());
    }

    #[test]
    fn should_find_multiple_templates() {
        let finder = TemplateFinder{
            root_dir: "./tests/fixtures/finder/"
        };
        let templates = finder.find_all();

        assert_eq!(3, templates.len());
    }
}
