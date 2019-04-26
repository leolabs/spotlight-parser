use std::collections::HashMap;
use std::io::{Error, Read, Seek, SeekFrom};

mod block;
mod header;

pub use block::*;
pub use header::*;

#[derive(Debug, Clone)]
pub struct Store {
  pub header: Header,
  //pub block0: Block0,
  //pub blocks: Vec<Block>,
  pub properties: HashMap<u32, Property>,
  //pub categories: HashMap<u32, String>,
}

impl Store {
  pub fn read_from(reader: &mut (impl Read + Seek)) -> Result<Self, Error> {
    let header = Header::read_from(reader)?;

    let property_blocks: Vec<PropertyBlock> = Self::parse_blocks(reader, &header)?;
    let properties: PropertyBlock = property_blocks.into_iter().collect();

    Ok(Store {
      header,
      properties: properties.data,
    })
  }

  fn parse_blocks<T: Block>(
    reader: &mut (impl Read + Seek),
    header: &Header,
  ) -> Result<Vec<T>, Error> {
    let property_block_metas =
      Meta::read_chain(reader, Some(T::BLOCK_TYPE), header.property_index)?;

    reader.seek(SeekFrom::Start(header.property_index as u64 * 4096))?;
    property_block_metas
      .into_iter()
      .map(|meta| {
        let next_index = meta.next_block_index;
        let block = T::from_meta(reader, meta)?;
        reader.seek(SeekFrom::Start(next_index as u64 * 4096))?;
        Ok(block)
      })
      .collect()
  }
}
