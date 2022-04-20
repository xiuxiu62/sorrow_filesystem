use std::io::{self, Read};

use crate::{util, Result, SORROW_FS_MAGIC};

#[derive(Debug)]
pub struct SuperBlock {
    pub magic: u32,
    pub block_size: u32,
    pub created_at: u64,
    pub modified_at: Option<u64>,
    pub last_mounted_at: Option<u64>,
    pub block_count: u32,
    pub inode_count: u32,
    pub free_blocks: u32,
    pub free_inodes: u32,
    pub groups: u32,
    pub data_blocks_per_group: u32,
    pub uid: u32,
    pub gid: u32,
    pub checksum: u32,
}

impl SuperBlock {
    pub fn new(block_size: u32, groups: u32, uid: u32, gid: u32) -> Result<Self> {
        let total_blocks = block_size * 8 * groups;

        Ok(Self {
            block_size,
            groups,
            uid,
            gid,
            magic: SORROW_FS_MAGIC,
            created_at: util::now()?,
            modified_at: None,
            last_mounted_at: None,
            free_blocks: total_blocks,
            free_inodes: total_blocks,
            block_count: total_blocks,
            inode_count: total_blocks,
            data_blocks_per_group: block_size * 8,
            checksum: 0,
        })
    }

    pub fn update_modified_at(&mut self) -> Result<()> {
        self.modified_at = Some(util::now()?);

        Ok(())
    }

    pub fn update_last_mounted_at(&mut self) -> Result<()> {
        self.last_mounted_at = Some(util::now()?);

        Ok(())
    }

    // fn checksum(&mut self) {
    //     self.checksum = 0;
    //     self.checksum = util::calculate_checksum(&self);
    // }

    // fn verify_checksum(&mut self) -> bool {
    //     let checksum = self.checksum;
    //     self.checksum = 0;
    //     let ok = checksum == util::calculate_checksum(&self);
    //     self.checksum = checksum;

    //     ok
    // }
}

impl Read for SuperBlock {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes = unsafe { util::struct_into_slice(&self) };
        std::mem::replace(&mut buf, &mut bytes);

        Ok(0)
    }
}
