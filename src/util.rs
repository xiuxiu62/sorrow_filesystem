use crate::error::Result;
use std::{
    io::Read,
    mem, slice,
    time::{self, SystemTime},
};

#[inline]
pub fn now() -> Result<u64> {
    Ok(SystemTime::now()
        .duration_since(time::UNIX_EPOCH)?
        .as_secs())
}

pub fn get_checksum<Reader>(reader: &mut Reader) -> Result<u32>
where
    Reader: Read,
{
    let mut contents: Vec<u8> = Vec::new();
    reader.read(&mut contents)?;

    let mut hasher = crc32fast::Hasher::new();
    hasher.update(contents.as_slice());

    Ok(hasher.finalize())
}

pub unsafe fn struct_into_slice<T>(p: &T) -> &[u8]
where
    T: Sized,
{
    slice::from_raw_parts((p as *const T) as *const u8, mem::size_of::<T>())
}
