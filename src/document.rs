use std::io;
use std::io::prelude::*;
use std::fs::File;

const INLINE_TAGS: [&'static str; 16] = [
    "area", "base", "br", "col", "command", "embed", "hr", "img",
    "input", "keygen", "link", "meta", "param", "source", "track", "wbr"];

const DOCTYPE_HTML: &'static str = "html";
const DOCTYPE_XML: &'static str = "xml";
const DOCTYPE_JSON: &'static str = "json";

pub struct Document {
    pub path: &'static str,
    pub contents: String,
    pub doctype: &'static str
}

impl Document {
    pub fn new(path: &'static str) -> Result<Document, io::Error> {
        let mut f = try!(File::open(path));

        let mut contents = String::new();
        try!(f.read_to_string(&mut contents));

        Ok(Document {
            path: path,
            contents: contents,
            doctype: "unknown"
        })
    }

    pub fn validate(&mut self) -> Option<&'static str> {
        if self.contents.len() == 0 {
            return Some("The file was empty");
        }

        if !self.contents.starts_with("doctype") && !self.contents.starts_with("extends") {
            return Some("The document must start with a 'doctype' or 'extends'");
        }

        // TODO: this isn't future proof, will need fixing
        match self.contents.lines().collect::<Vec<_>>()[0].trim() {
            "doctype html" => self.doctype = DOCTYPE_HTML,
            "doctype xml" => self.doctype = DOCTYPE_XML,
            "doctype json" => self.doctype = DOCTYPE_JSON,
            _ => return Some("Unknown 'doctype' supplied")
        }

        None
    }

    pub fn render_doctype(&self, node: &DocumentNode, pretty: bool) -> Option<String> {
        if node.tokens[0] == "doctype" {
            if let Some(doctype) = generate_doctype(&node.content) {
                if pretty {
                    return Some(format!("{}\n", doctype));
                }

                return Some(doctype);
            }
        } /*else if n.tokens[0] == "extends" {
            // TODO: when extends is added
        }*/
        None
    }
}

pub struct DocumentNode {
    pub depth: usize,
    pub tokens: Vec<String>,
    pub content: String,
    pub is_self_closing: bool,
    pub ignore_sub_content: bool
}

impl DocumentNode {
    pub fn from_content(indent: usize, content: String) -> DocumentNode {
        DocumentNode {
            depth: indent,
            tokens: vec!["|".to_string()],
            content: content,
            is_self_closing: false,
            ignore_sub_content: true
        }
    }

    pub fn from(line: &str, indent: usize, doctype: &str) -> Option<DocumentNode> {
        let (tokens, content) = match split_tokens(String::from(line.trim())) {
            Some(result) => result,
            None => return None
        };

        let mut is_self_closing = false;
        let mut ignore_sub_content = false;
        let last_token = &*tokens.last().unwrap().to_string();

        if tokens[0] == "//" || tokens[0] == "|" {
            ignore_sub_content = true;
        } else if last_token == "/" {
            is_self_closing = true;
        }

        if doctype == DOCTYPE_HTML {
            if tokens[0] == "style" || tokens[0] == "script" {
                ignore_sub_content = true;
            } else if INLINE_TAGS.contains(&&*tokens[0].to_string()) {
                is_self_closing = true;
            }
        }

        Some(DocumentNode {
            depth: indent,
            tokens: tokens,
            content: content,
            is_self_closing: is_self_closing,
            ignore_sub_content: ignore_sub_content
        })
    }

    pub fn render(&self, pretty: bool) -> String {
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

    pub fn render_open(&self, pretty: bool) -> String {
        let tag_tokens = &self.tokens;
        let mut output = String::new();

        if tag_tokens[0] == "//" {
            output.push_str("<!--");
        } else if tag_tokens[0] == "|" {
            output.push_str(&self.content.to_string());
        } else {
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
        }

        if pretty {
            return pretty_print(&output, self.depth);
        }

        output
    }

    pub fn render_end(&self, pretty: bool) -> String {
        let output = match self.tokens[0].as_ref() {
            "//" => "-->".to_string(),
            "|" => String::new(),
            _ => format!("</{}>", &self.tokens[0])
        };

        if pretty && !output.is_empty() {
            return pretty_print(&output, self.depth);
        }

        output
    }
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
        tokens.push("//".to_string());
        content.push_str(&s[2..].trim().to_string());
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
