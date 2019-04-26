use byteorder::{ReadBytesExt, LE};
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum Type {
  Metadata,
  Property,
  Category,
  Index,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct Index {
  pub offset_index: u32,
  pub dest_block_size: u32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Block0 {
  pub physical_size: u32,
  pub indices: Vec<Index>,
}

// TODO Wie kann man die Datengröße auch im Typ festlegen?
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Meta {
  pub physical_size: u32,
  pub logical_size: u32,
  pub block_type: Option<Type>,
  pub next_block_index: u32,
}

impl Type {
  pub fn read_from(reader: &mut (impl Read)) -> Result<Option<Self>, Error> {
    match reader.read_u32::<LE>()? {
      0x09 => Ok(Some(Type::Metadata)),
      0x11 => Ok(Some(Type::Property)),
      0x21 => Ok(Some(Type::Category)),
      0x81 => Ok(Some(Type::Index)),
      0x00 => Ok(None),
      0x41 => Ok(None),
      _ => Err(Error::new(ErrorKind::InvalidData, "Block Type is unknown")),
    }
  }
}

impl Index {
  pub fn read_from(reader: &mut (impl Read + Seek)) -> Result<Self, Error> {
    reader.seek(SeekFrom::Current(8))?;

    let offset_index = reader.read_u32::<LE>()?;
    let dest_block_size = reader.read_u32::<LE>()?;

    Ok(Index {
      offset_index,
      dest_block_size,
    })
  }
}

impl Block0 {
  pub fn read_from(reader: &mut (impl Read + Seek)) -> Result<Self, Error> {
    reader.seek(SeekFrom::Start(4096))?;

    if !Self::is_valid_signature(reader)? {
      return Err(Error::new(
        ErrorKind::InvalidData,
        "Block0 Signature is invalid",
      ));
    }

    let physical_size = reader.read_u32::<LE>()?;
    let item_count = reader.read_u32::<LE>()?;
    reader.seek(SeekFrom::Current(8))?;

    let indices = (0..item_count)
      .map(|_| Index::read_from(reader))
      .collect::<Result<_, _>>()?;

    Ok(Block0 {
      physical_size,
      indices,
    })
  }

  fn is_valid_signature(reader: &mut (impl Read)) -> Result<bool, Error> {
    let signature = reader.read_u32::<LE>()?;

    Ok(match signature {
      0x64626D31 | 0x64626D32 => true,
      _ => false,
    })
  }
}

impl Meta {
  pub fn read_from(reader: &mut (impl Read + Seek)) -> Result<Self, Error> {
    if !Self::is_valid_signature(reader)? {
      return Err(Error::new(
        ErrorKind::InvalidData,
        "Block Signature is invalid",
      ));
    }

    let physical_size = reader.read_u32::<LE>()?;
    let logical_size = reader.read_u32::<LE>()?;
    let block_type = Type::read_from(reader)?;
    reader.seek(SeekFrom::Current(4))?;
    let next_block_index = reader.read_u32::<LE>()?;
    reader.seek(SeekFrom::Current(8))?;
    reader.seek(SeekFrom::Start(0))?;

    Ok(Meta {
      physical_size,
      logical_size,
      block_type,
      next_block_index,
    })
  }

  pub fn read_chain(
    reader: &mut (impl Read + Seek),
    expected_type: Option<Type>,
    mut index: u32,
  ) -> Result<Vec<Meta>, Error> {
    let mut results = Vec::new();

    while index != 0 {
      reader.seek(SeekFrom::Start((index * 4096) as u64))?;
      let block = Meta::read_from(reader)?;

      if block.block_type != expected_type {
        return Err(Error::new(
          ErrorKind::InvalidData,
          "Block Type doesn't match",
        ));
      }

      index = block.next_block_index;
      results.push(block);
    }

    Ok(results)
  }

  fn is_valid_signature(reader: &mut (impl Read)) -> Result<bool, Error> {
    Ok(reader.read_u32::<LE>()? == 0x64627032)
  }
}
