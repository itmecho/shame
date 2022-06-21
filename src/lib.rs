pub mod sha256;

pub trait Hasher {
    fn generate_hash(&self, data: Vec<u8>) -> String;
}
