use std::collections::HashMap;
use parking_lot::RwLock;

const PAGE_SIZE: usize = 4096;
const PAGE_MASK: u64 = !(PAGE_SIZE as u64 - 1);

pub struct PageTable {
    entries: RwLock<HashMap<u64, Page>>,
    permissions: RwLock<HashMap<u64, PagePermissions>>,
}

#[derive(Clone, Copy)]
pub struct PagePermissions {
    read: bool,
    write: bool,
    execute: bool,
}

pub struct Page {
    data: Box<[u8; PAGE_SIZE]>,
    dirty: bool,
    accessed: bool,
}

impl PageTable {
    pub fn new() -> Self {
        PageTable {
            entries: RwLock::new(HashMap::new()),
            permissions: RwLock::new(HashMap::new()),
        }
    }

    pub fn read(&self, addr: u64) -> Result<u64, MemoryError> {
        let page_addr = addr & PAGE_MASK;
        let offset = (addr & !PAGE_MASK) as usize;
        
        let perms = self.permissions.read();
        if let Some(perm) = perms.get(&page_addr) {
            if !perm.read {
                return Err(MemoryError::AccessViolation);
            }
        }
        
        let entries = self.entries.read();
        if let Some(page) = entries.get(&page_addr) {
            Ok(u64::from_le_bytes(page.data[offset..offset+8].try_into()?))
        } else {
            Err(MemoryError::PageFault)
        }
    }
} 