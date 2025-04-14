use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct Error;

pub struct LineTableBuf<'a> {
    offset: usize,
    table: &'a [u8],
}

impl<'a> LineTableBuf<'a> {
    pub fn new(it: &'a [u8]) -> Self {
        Self {
            table: it,
            offset: 0,
        }
    }
}

impl LineTableBuf<'_> {
    pub fn empty(&self) -> bool {
        self.offset >= self.table.len()
    }

    pub fn u8(&mut self) -> Result<u8, Error> {
        match self.table.get(self.offset) {
            None => Err(Error),
            Some(v) => {
                self.offset += 1;
                Ok(*v)
            }
        }
    }

    pub fn varint(&mut self) -> Result<usize, Error> {
        let mut ret: usize;

        let mut byte = self.u8()?;
        let mut shift = 0;
        ret = (byte & 63) as usize;
        let mut j = 0;
        while byte & 64 != 0 {
            if j >= 10 {
                return Err(Error);
            }
            byte = self.u8()?;
            shift += 6;
            let add = ((byte & 63) as usize).checked_shl(shift).ok_or(Error)?;
            ret = ret.checked_add(add).ok_or(Error)?;
            j += 1;
        }
        Ok(ret)
    }

    pub fn signed_varint(&mut self) -> Result<isize, Error> {
        let unsigned_val = self.varint()?;
        if unsigned_val & 1 != 0 {
            Ok(-((unsigned_val >> 1) as isize))
        } else {
            Ok((unsigned_val >> 1) as isize)
        }
    }
}
