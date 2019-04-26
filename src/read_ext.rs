use byteorder::ReadBytesExt;
use std::io::{Error, ErrorKind, Read};

pub trait ReadExt: Read {
  fn read_zero_terminated(&mut self) -> Result<(String, usize), Error> {
    let mut name = Vec::new();
    let mut read = 0;

    loop {
      let character = self.read_u8()?;
      read += 1;
      if character == 0 {
        break;
      }
      name.push(character);
    }

    let res = String::from_utf8(name).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    Ok((res, read))
  }
}

impl<T: Read> ReadExt for T {}
