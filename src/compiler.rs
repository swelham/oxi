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

        let output = doc.compile(pretty);

        Ok(output)
    }
}
