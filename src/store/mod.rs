use byteorder::{ReadBytesExt, LE};
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
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Property {
  pub name: String,
  pub value_type: u8,
  pub prop_type: u8,
}

impl Store {
  pub fn read_from(reader: &mut (impl Read + Seek)) -> Result<Self, Error> {
    let header = Header::read_from(reader)?;
    let block0 = Block0::read_from(reader)?;

    let property_blocks = Block::read_chain(
      reader,
      Some(Type::Property),
      header.property_index,
      header.block_size,
    )?;
    let properties = Self::parse_properties(property_blocks)?;

    Ok(Store { header, properties })
  }

  fn parse_properties(blocks: Vec<Block>) -> Result<HashMap<u32, Property>, Error> {
    let mut property_map: HashMap<u32, Property> = HashMap::new();

    for mut block in blocks {
      block.data.seek(SeekFrom::Start(32))?;

      while block.data.position() < block.logical_size as u64 {
        let index = block.data.read_u32::<LE>()?;
        let value_type = block.data.read_u8()?;
        let prop_type = block.data.read_u8()?;

        let mut name = String::new();

        while block.data.position() < block.logical_size as u64 {
          let character = block.data.read_u8()? as char;
          if character == '\x00' {
            break;
          }
          name.push(character);
        }

        property_map.insert(
          index,
          Property {
            value_type,
            prop_type,
            name,
          },
        );
      }
    }

    Ok(property_map)
  }
}
