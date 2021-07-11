use std::cell::RefCell;
use std::rc::Rc;

#[derive(Copy, Clone)]
pub enum StorageMode {
    None,
    R,
    RW,
}

pub struct Storage {
    data: Rc<RefCell<Vec<u8>>>,
    address: Option<u16>,
    storage_mode_8: StorageMode,
    storage_mode_16: StorageMode,
}

impl Storage {
    pub fn new(size: u16, storage_mode_8: StorageMode, storage_mode_16: StorageMode) -> Self {
        Self::initialize(
            Rc::new(RefCell::new(vec![0; size as usize])),
            None,
            storage_mode_8,
            storage_mode_16,
        )
    }

    pub fn from_data(
        data: Vec<u8>,
        storage_mode_8: StorageMode,
        storage_mode_16: StorageMode,
    ) -> Self {
        Self::initialize(
            Rc::from(RefCell::from(data)),
            None,
            storage_mode_8,
            storage_mode_16,
        )
    }

    pub fn duplicate(&self, storage_mode_8: StorageMode, storage_mode_16: StorageMode) -> Self {
        Self::initialize(
            self.data.clone(),
            self.address,
            storage_mode_8,
            storage_mode_16,
        )
    }

    pub fn view(&self, address: u16) -> Self {
        Self::initialize(
            self.data.clone(),
            match self.address {
                Some(base) => Some(base + address),
                None => Some(address),
            },
            self.storage_mode_8,
            self.storage_mode_16,
        )
    }

    fn initialize(
        data: Rc<RefCell<Vec<u8>>>,
        address: Option<u16>,
        storage_mode_8: StorageMode,
        storage_mode_16: StorageMode,
    ) -> Self {
        Self {
            data,
            address,
            storage_mode_8,
            storage_mode_16,
        }
    }

    pub fn read_u8(&self) -> u8 {
        if let StorageMode::None = self.storage_mode_8 {
            panic!("can't read u8 from this storage");
        }
        let address = self.address.expect("storage has no address");
        let data = &self.data.borrow()[..];
        data[address as usize]
    }

    pub fn read_u16(&self) -> u16 {
        if let StorageMode::None = self.storage_mode_16 {
            panic!("can't read u16 from this storage");
        }
        let address = self.address.expect("storage has no address");
        let data = &self.data.borrow()[..];
        let first_byte = data[address as usize] as u16;
        let second_byte = data[(address + 1) as usize] as u16;
        second_byte << 8 | first_byte
    }

    pub fn write_u8(&mut self, value: u8) {
        if let StorageMode::R | StorageMode::None = self.storage_mode_8 {
            panic!("can't write u8 to this storage");
        }
        let address = self.address.expect("storage has no address");
        let data = &mut self.data.borrow_mut()[..];
        data[address as usize] = value;
    }

    pub fn write_u16(&mut self, value: u16) {
        if let StorageMode::R | StorageMode::None = self.storage_mode_16 {
            panic!("can't write u16 to this storage");
        }
        let address = self.address.expect("storage has no address");
        let data = &mut self.data.borrow_mut()[..];
        data[address as usize] = value as u8;
        data[(address + 1) as usize] = (value >> 8) as u8;
    }
}
