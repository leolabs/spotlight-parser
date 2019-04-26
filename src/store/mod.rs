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
  pub categories: HashMap<u32, Category>,
}

impl Store {
  pub fn read_from(reader: &mut (impl Read + Seek)) -> Result<Self, Error> {
    let header = Header::read_from(reader)?;

    let property_blocks: Vec<PropertyBlock> = Self::parse_blocks(reader, header.property_index)?;
    let properties: PropertyBlock = property_blocks.into_iter().collect();

    let category_blocks: Vec<CategoryBlock> = Self::parse_blocks(reader, header.category_index)?;
    let categories: CategoryBlock = category_blocks.into_iter().collect();

    Ok(Store {
      header,
      properties: properties.data,
      categories: categories.data,
    })
  }

  fn parse_blocks<T: Block>(reader: &mut (impl Read + Seek), offset: u32) -> Result<Vec<T>, Error> {
    let property_block_metas = Meta::read_chain(reader, Some(T::BLOCK_TYPE), offset)?;

    reader.seek(SeekFrom::Start(offset as u64 * 4096))?;
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
