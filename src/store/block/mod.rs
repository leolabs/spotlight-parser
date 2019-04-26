use std::io::{Error, Read, Seek};

mod meta;
mod properties;

pub use self::meta::*;
pub use self::properties::*;

pub trait Block {
  const BLOCK_TYPE: Type;
  fn from_meta(reader: &mut (impl Read + Seek), meta: Meta) -> Result<Self, Error>
  where
    Self: std::marker::Sized;
}
