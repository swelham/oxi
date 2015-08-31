use document::Document;

pub struct Compiler;

impl Compiler {
    pub fn compile(file_path: &'static str, pretty: bool) -> Result<String, String> {
        let mut doc = match Document::new(file_path) {
            Ok(doc) => doc,
            Err(e) => return Err(format!("{}: {}", e, file_path))
        };

        if let Some(e) = doc.validate() {
            return Err(format!("{}: {}", e, file_path));
        }

        let output = self.compile(doc, pretty);

        Ok(output)
    }

    pub fn compile_document(doc: Document, pretty: bool) -> String {
        let nodes = parse(self);

        let mut parent_stack: Vec<&DocumentNode> = Vec::new();
        let mut output = String::new();
        let len = nodes.len();
        let mut i = 0;

        for n in &nodes {
            if i == 0 {
                if n.tokens[0] == "doctype" {
                    if let Some(doctype) = generate_doctype(&n.content) {
                        output.push_str(&doctype.to_string());

                        if pretty {
                            output.push('\n');
                        }
                    } else {
                        // TODO: proper error handling
                        panic!("Unknown 'doctype' suppied");
                    }
                } /*else if n.tokens[0] == "extends" {
                    // TODO: when extends is added
                }*/

                i += 1;
                continue;
            }

            // TODO: sort this code nesting out, it's too deep
            let has_next = i + 1 < len;
            let has_sub = has_next && nodes[i + 1].depth > n.depth;

            if has_sub {
                output.push_str(&n.render_open(pretty).to_string());
                parent_stack.push(n);
            } else {
                output.push_str(&n.render(pretty).to_string());

                if has_next && nodes[i + 1].depth < n.depth {
                    loop {
                        match parent_stack.pop() {
                            None =>  { break; },
                            Some(p) => {
                                output.push_str(&p.render_end(pretty).to_string());

                                if p.depth == nodes[i + 1].depth {
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            if !has_next && parent_stack.len() > 0 {
                loop {
                    match parent_stack.pop() {
                        None =>  { break; },
                        Some(p) => {
                            output.push_str(&p.render_end(pretty).to_string());
                        }
                    }
                }
            }

            if !pretty && has_next {
                let next = &nodes[i + 1];

                if n.tokens[0] == "|" && next.tokens[0] == "|" {
                    output.push(' ');
                }
            }

            i += 1;
        }

        output
    }
}
