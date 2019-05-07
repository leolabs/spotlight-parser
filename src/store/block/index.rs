use crate::read_ext::ReadExt;
use byteorder::{ReadBytesExt, LE};
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};
use std::iter::FromIterator;

use super::meta::Meta;
use super::Block;
use super::Type;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Index {
  pub name: String,
  pub lang: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IndexBlock {
  pub data: HashMap<u32, Category>,
}

impl Block for IndexBlock {
  const BLOCK_TYPE: Type = Type::Index;

  fn from_meta(reader: &mut (impl Read + Seek), meta: Meta) -> Result<Self, Error> {
    let mut index_map: HashMap<u32, Category> = HashMap::new();

    let mut pos = 32;
    reader.seek(SeekFrom::Current(32))?;

    while pos < meta.logical_size as usize {
      let index = reader.read_u32::<LE>()?;
      let (name, len) = reader.read_zero_terminated()?;

      if category_map.get(&index).is_some() {
        return Err(Error::new(
          ErrorKind::AlreadyExists,
          "This category item already exists",
        ));
      }

      index_map.insert(
        index,
        Category {
          name: name_parts[0].to_owned(),
          lang: if name_parts.len() > 1 {
            Some(name_parts[1].to_owned())
          } else {
            None
          },
        },
      );
    }

    Ok(Self { data: category_map })
  }
}

impl FromIterator<CategoryBlock> for CategoryBlock {
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = Self>,
  {
    let data = iter.into_iter().flat_map(|b| b.data.into_iter()).collect();
    CategoryBlock { data }
  }
}
