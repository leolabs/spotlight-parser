use byteorder::{ReadBytesExt, LE};
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};

const MAGIC_NUMBER: &[u8] = b"\x38\x74\x73\x64";

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Header {
  pub flags: u32,
  pub header_size: u32,
  pub block_size: u32,
  pub block0_size: u32,
  pub original_path: String,
  pub property_index: u32,
  pub category_index: u32,
  pub item_kind_index: u32,
  pub user_tags_index: u32,
}

impl Header {
  pub fn read_from(reader: &mut (impl Read + Seek)) -> Result<Self, Error> {
    if !Self::is_valid_file(reader)? {
      return Err(Error::new(ErrorKind::InvalidData, "Magic number mismatch"));
    }

    let flags = reader.read_u32::<LE>()?;
    reader.seek(SeekFrom::Start(36))?;
    let header_size = reader.read_u32::<LE>()?;
    let block0_size = reader.read_u32::<LE>()?;
    let block_size = reader.read_u32::<LE>()?;
    let property_index = reader.read_u32::<LE>()?;
    let category_index = reader.read_u32::<LE>()?;
    reader.seek(SeekFrom::Start(4))?;
    let item_kind_index = reader.read_u32::<LE>()?;
    let user_tags_index = reader.read_u32::<LE>()?;
    let mut original_path = String::new();
    reader.seek(SeekFrom::Start(0x144))?;
    reader
      .by_ref()
      .take(256)
      .read_to_string(&mut original_path)?;

    original_path = original_path.trim_end_matches('\0').to_owned();

    reader.seek(SeekFrom::Start(4096))?;

    Ok(Header {
      flags,
      header_size,
      block0_size,
      block_size,
      property_index,
      category_index,
      item_kind_index,
      user_tags_index,
      original_path,
    })
  }

  fn is_valid_file(reader: &mut (impl Read + Seek)) -> Result<bool, Error> {
    let mut tmp = [0; 4];
    reader.seek(SeekFrom::Start(0))?;
    reader.read_exact(&mut tmp)?;
    Ok(tmp == MAGIC_NUMBER)
  }
}
