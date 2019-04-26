use crate::read_ext::ReadExt;
use byteorder::{ReadBytesExt, LE};
use std::collections::HashMap;
use std::io::{Error, Read, Seek, SeekFrom};
use std::iter::FromIterator;

use super::meta::Meta;
use super::Block;
use super::Type;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Property {
  pub name: String,
  pub value_type: u8,
  pub prop_type: u8,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PropertyBlock {
  pub data: HashMap<u32, Property>,
}

impl Block for PropertyBlock {
  const BLOCK_TYPE: Type = Type::Property;

  fn from_meta(reader: &mut (impl Read + Seek), meta: Meta) -> Result<Self, Error> {
    let mut property_map: HashMap<u32, Property> = HashMap::new();

    let mut pos = 32;
    reader.seek(SeekFrom::Current(32))?;

    while pos < meta.logical_size as usize {
      let index = reader.read_u32::<LE>()?;
      let value_type = reader.read_u8()?;
      let prop_type = reader.read_u8()?;
      let (name, len) = reader.read_zero_terminated()?;

      pos += 6 + len + 1;

      property_map.insert(
        index,
        Property {
          value_type,
          prop_type,
          name,
        },
      );
    }

    Ok(Self { data: property_map })
  }
}

impl FromIterator<PropertyBlock> for PropertyBlock {
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = Self>,
  {
    let data = iter.into_iter().flat_map(|b| b.data.into_iter()).collect();
    PropertyBlock { data }
  }
}
