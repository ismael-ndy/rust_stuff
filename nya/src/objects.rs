use flate2::Compression;
use std::io::prelude::*
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;


pub trait NyaObject {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(&self, data: Vec<u8>);
}

pub struct Blob {
    content: String,
    content_size: u8,
}

impl Blob {
    pub fn new(content: String) -> Self {
        let content_size = content.len() as u8;
        Self {
            content: content,
            content_size: content_size,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.content.as_bytes()
    }
}

impl NyaObject for Blob {
    fn serialize(&self) -> Vec<u8> {
        let header = format!("blob {}\0", self.content_size);
        let content = [header, self.content].concat().as_bytes();

        // ZLIB

        Vec::new()
    }

    fn deserialize(&self, data: Vec<u8>) {}
}
