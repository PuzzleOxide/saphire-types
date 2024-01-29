use std::io::Write;

use crate::block::Block;
use flate2::{write::GzEncoder, Compression};
use base64::{engine::general_purpose, Engine as _};

/// A diamondfire template, used to store a list of blocks and to generate code.
pub struct Template {
    /// The name of the template.
    pub name: String,
    /// The list of blocks in the template.
    pub blocks: Vec<Block>,
}

impl Template {
    /// Create a new template.
    pub fn new(name: String, blocks: Vec<Block>) -> Self {
        Self {
            name,
            blocks,
        }
    }

    /// Generate code from the template.
    pub fn compile(&self) -> String {
        let mut code = String::new();
        for block in &self.blocks {
            code.push_str(&block.compile());
        }
        
        let mut e = GzEncoder::new(Vec::new(), Compression::default());
        e.write_all(code.as_bytes()).unwrap();
        general_purpose::STANDARD.encode(e.finish().unwrap()) //TODO: wrap in item nbt
    }
}