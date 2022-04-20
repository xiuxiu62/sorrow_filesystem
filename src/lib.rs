mod error;
mod super_block;
mod util;

pub use crate::{
    error::{Error, Result},
    super_block::SuperBlock,
};

pub(crate) const SORROW_FS_MAGIC: u32 = 0x64627b00;
