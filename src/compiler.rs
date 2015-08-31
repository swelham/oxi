use document::{Document, DocumentNode};

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

        let output = try!(Compiler::compile_document(&doc, pretty));

        Ok(output)
    }

    fn compile_document(doc: &Document, pretty: bool) -> Result<String, String> {
        let nodes = parse(doc);

        let mut parent_stack: Vec<&DocumentNode> = Vec::new();
        let mut output = String::new();
        let len = nodes.len();
        let mut i = 0;

        for n in &nodes {
            if i == 0 {
                if let Some(doctype) = doc.render_doctype(n, pretty) {
                    output.push_str(&doctype.to_string());
                }

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

        Ok(output)
    }
}

fn parse(doc: &Document) -> Vec<DocumentNode> {
    let mut nodes: Vec<DocumentNode> = Vec::new();
    let mut parsable_indent = 0;
    let mut mode = 0;

    for line in doc.contents.lines() {
        if line.is_empty() {
            continue;
        }

        let mut indent: usize = 0;

        for c in line.chars() {
            if c.is_whitespace() {
                indent += 1;
            } else {
                break;
            }
        }

        if mode == 2 && indent > parsable_indent {
            continue;
        } else if mode == 1 && indent > parsable_indent {
            nodes.push(DocumentNode::from_content(indent, line.trim().to_string()));
            continue;
        }

        if let Some(node) = DocumentNode::from(line, indent, doc.doctype) {
            if node.ignore_sub_content {
                mode = 1;
            } else {
                mode = 0;
            }

            nodes.push(node);
        } else {
            mode = 2;
        }

        parsable_indent = indent;
    }

    nodes
}
