
// TODO: this will actually not return String but a vec of nodes
fn parse(template: String) -> Option<String> {
    let doctype = resolve_doctype(template);

    // TODO: create parser and build up node tree
}

fn resolve_doctype(template: String) -> Option<String> {
    let mut doctype = String::new();

    for line in template.lines() {
        if !line.starts_with("@") {
            break;
        }

        if line.starts_with("@doctype") {
            doctype = line[8..].trim().to_string();
            break;
        }
    }

    if doctype == "" {
        return None;
    }

    Some(doctype)
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_resolve_html_type() {
        let templates = vec![
            "@doctype html",
            "@something\n@doctype html"
        ];

        for template in templates {
            let doctype = super::resolve_doctype(template.to_string()).unwrap();
            assert_eq!(doctype, "html");
        }
    }

    #[test]
    fn should_resolve_no_type() {
        let templates = vec![
            "no type specified",
            "none\n\n@doctype html"
        ];

        for template in templates {
            let doctype = super::resolve_doctype(template.to_string());
            assert_eq!(doctype, None);
        }
    }
}
