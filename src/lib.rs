use std::io;
use std::io::prelude::*;
use std::fs::File;

const INLINE_TAGS: [&'static str; 16] = [
    "area", "base", "br", "col", "command", "embed", "hr", "img",
    "input", "keygen", "link", "meta", "param", "source", "track", "wbr"];

const DOCTYPE_HTML: &'static str = "html";
const DOCTYPE_XML: &'static str = "xml";
const DOCTYPE_JSON: &'static str = "json";

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

        let output = doc.compile(pretty);

        Ok(output)
    }
}

// TODO: move document into its own file
struct DocumentNode {
    depth: usize,
    tokens: Vec<String>,
    content: String,
    is_self_closing: bool
}

impl DocumentNode {
    fn from(line: &str, doctype: &str) -> Option<DocumentNode> {
        let mut indent = 0;

        for c in line.chars() {
            if c.is_whitespace() {
                indent += 1;
            } else {
                break;
            }
        }

        let (tokens, content) = match split_tokens(String::from(line.trim())) {
            Some(result) => result,
            None => return None
        };

        let mut self_closing = false;
        let last_token = &*tokens.last().unwrap().to_string();

        if last_token == "/" || (doctype == DOCTYPE_HTML && INLINE_TAGS.contains(&&*tokens[0].to_string())) {
            self_closing = true;
        }

        Some(DocumentNode {
            depth: indent,
            tokens: tokens,
            content: content,
            is_self_closing: self_closing
        })
    }

    fn render(&self, pretty: bool) -> String {
        let mut output = String::new();

        if self.tokens[0] == "|" {
            output.push_str(&self.content.to_string());

            if pretty {
                return pretty_print(&output, self.depth);
            } else {
                return output;
            }
        }

        output.push_str(&self.render_open(false).to_string());

        if self.is_self_closing {
            if pretty {
                return pretty_print(&output, self.depth);
            }

            return output;
        }

        if !self.content.is_empty() {
            output.push_str(&self.content.to_string());
        }

        output.push_str(&self.render_end(false).to_string());

        if pretty {
            return pretty_print(&output, self.depth);
        }

        output
    }

    fn render_open(&self, pretty: bool) -> String {
        let tag_tokens = &self.tokens;
        let mut output = String::new();

        output.push_str(&format!("<{}", tag_tokens[0]).to_string());

        if tag_tokens.len() > 1 {
            let mut classes = String::new();
            let mut attributes = String::new();

            for t in &tag_tokens[1..] {
                if t.starts_with('#') {
                    output.push_str(&format!(" id=\"{}\"", t[1..].to_string()))
                } else if t.starts_with('.') {
                    classes.push_str(&format!(" {}", t[1..].to_string()));
                } else if t.starts_with('(') {
                    let attrs: Vec<_> = t[1..].split(',').collect();

                    attributes.push_str(&attrs.concat().to_string());
                }
            }

            if !classes.is_empty() {
                output.push_str(&format!(" class=\"{}\"", classes.trim()));
            }

            if !attributes.is_empty() {
                output.push_str(&format!(" {}", attributes).to_string());
            }
        }

        if self.is_self_closing {
            output.push('/');
        }

        output.push('>');

        if pretty {
            return pretty_print(&output, self.depth);
        }

        output
    }

    fn render_end(&self, pretty: bool) -> String {
        let output = format!("</{}>", &self.tokens[0]);

        if pretty {
            return pretty_print(&output, self.depth);
        }

        output
    }
}

struct Document {
    path: &'static str,
    contents: String,
    doctype: &'static str
}

impl Document {
    fn new(path: &'static str) -> Result<Document, io::Error> {
        let mut f = try!(File::open(path));

        let mut contents = String::new();
        try!(f.read_to_string(&mut contents));

        Ok(Document {
            path: path,
            contents: contents,
            doctype: "unknown"
        })
    }

    fn validate(&mut self) -> Option<&'static str> {
        if self.contents.len() == 0 {
            return Some("The file was empty");
        }

        if !self.contents.starts_with("doctype") && !self.contents.starts_with("extends") {
            return Some("The document must start with a 'doctype' or 'extends'");
        }

        // TODO: this isn't future proof, will need fixing
        match self.contents.lines().collect::<Vec<_>>()[0] {
            "doctype html" => self.doctype = DOCTYPE_HTML,
            "doctype xml" => self.doctype = DOCTYPE_XML,
            "doctype json" => self.doctype = DOCTYPE_JSON,
            _ => return Some("Unknown 'doctype' supplied")
        }

        None
    }

    fn compile(self, pretty: bool) -> String {
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

            i += 1;
        }

        output
    }
}

fn parse(doc: Document) -> Vec<DocumentNode> {
    let mut nodes: Vec<DocumentNode> = Vec::new();

    for line in doc.contents.lines() {
        if line.is_empty() {
            continue;
        }

        if let Some(node) = DocumentNode::from(line, doc.doctype) {
            nodes.push(node);
        }
    }

    nodes
}

fn split_tokens(s: String) -> Option<(Vec<String>, String)> {
    let mut tokens: Vec<String> = Vec::new();

    let mut start = 0;
    let len = s.len();
    let mut mode = 0;
    let mut content = String::new();

    if s.starts_with("|") {
        tokens.push("|".to_string());
        content.push_str(&s[1..].trim().to_string());
    } else if s.starts_with("//-") {
        return None;
    } else if s.starts_with("//") {
        // tokens.push("/");
        // content.push_str(&s[2..].trim().to_string());
    }

    if tokens.len() > 0 {
        return Some((tokens, content));
    }

    for (i, c) in s.chars().enumerate() {
        if c.is_whitespace() && mode == 0 {
            tokens.push(s[start..i].to_string());
            content.push_str(&s[i..len].trim().to_string());
            break;
        }

        if c == ')' {
            if mode != 1 {
                // TODO: proper error handling
                panic!("Invalid attribute closing brace");
            }

            tokens.push(s[start..i].to_string());

            mode = 0;
            start = i;
        } else {
            if c == '/' && mode == 0 {
                tokens.push(s[start..i].to_string());
                tokens.push("/".to_string());
                start = i + 1;
            } else if ((c == '#' || c == '.' || c == '(') && mode == 0) || i == len - 1 {
                if i == len - 1 {
                    tokens.push(s[start..].to_string());
                } else if i > start {
                    tokens.push(s[start..i].to_string());
                }

                if c == '(' {
                    mode = 1;
                }

                start = i;
            }
        }
    }

    // TODO: pretty sure this can be done using a pattern in the starts_with
    if tokens.len() > 0 && (tokens[0].starts_with(".") || tokens[0].starts_with("#")) {
        tokens.insert(0, "div".to_string());
    }

    Some((tokens, content))
}

// TODO: need to abstract this stuff to support multiple doctypes
fn generate_doctype(content: &String) -> Option<String> {
    return match content.as_ref() {
        "html" => Some(format!("<!DOCTYPE {}>", content)),
        "xml" => Some("<?xml version=\"1.0\" encoding=\"utf-8\" ?>".to_string()),
        _ => None
    };
}

// TODO: look at making this a macro
// TODO: reduce if statements when using this by applying it always with a flag to indicate which format to use
fn pretty_print(content: &String, depth: usize) -> String {
    let mut output = String::new();

    for _ in 0..depth {
        output.push(' ');
    }

    output.push_str(&content.to_string());
    output.push('\n');
    output
}
