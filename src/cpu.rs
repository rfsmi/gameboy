use std::{cell::RefCell, ops::Range, rc::Rc};

fn read_u8(data: &[u8], address: u16) -> u8 {
    data[address as usize]
}

fn read_u16(data: &[u8], address: u16) -> u16 {
    let first_byte = data[address as usize] as u16;
    let second_byte = data[(address + 1) as usize] as u16;
    second_byte << 8 | first_byte
}

fn write_u8(data: &mut [u8], address: u16, value: u8) {
    data[address as usize] = value;
}

fn write_u16(data: &mut [u8], address: u16, value: u16) {
    data[address as usize] = value as u8;
    data[(address + 1) as usize] = (value >> 8) as u8;
}

struct Storage {
    data: Rc<RefCell<Vec<u8>>>,
    range: Range<usize>,
    read_u8_impl: Option<fn(&[u8], u16) -> u8>,
    read_u16_impl: Option<fn(&[u8], u16) -> u16>,
    write_u8_impl: Option<fn(&mut [u8], u16, u8)>,
    write_u16_impl: Option<fn(&mut [u8], u16, u16)>,
}

impl Storage {
    fn new(size: u16, eight_bit: bool, sixteen_bit: bool) -> Self {
        Self::initialize(
            Rc::new(RefCell::new(vec![0; size as usize])),
            0..size as usize,
            eight_bit,
            sixteen_bit,
        )
    }

    fn duplicate(&self, eight_bit: bool, sixteen_bit: bool) -> Self {
        Self::initialize(self.data, self.range, eight_bit, sixteen_bit)
    }

    fn view(&self, range: Range<usize>) -> Self {
        Self::initialize(
            self.data,
            range,
            self.read_u8_impl.is_some(),
            self.read_u16_impl.is_some(),
        )
    }

    fn initialize(
        data: Rc<RefCell<Vec<u8>>>,
        range: Range<usize>,
        eight_bit: bool,
        sixteen_bit: bool,
    ) -> Self {
        Self {
            data,
            range,
            read_u8_impl: if eight_bit { Some(read_u8) } else { None },
            read_u16_impl: if sixteen_bit { Some(read_u16) } else { None },
            write_u8_impl: if eight_bit { Some(write_u8) } else { None },
            write_u16_impl: if sixteen_bit { Some(write_u16) } else { None },
        }
    }

    fn read_u8(&self, address: u16) -> u8 {
        match self.read_u8_impl.clone() {
            Some(f) => f(&mut (*self.data).borrow()[self.range], address),
            None => panic!("can't read u8 to this storage"),
        }
    }

    fn read_u16(&self, address: u16) -> u16 {
        match self.read_u16_impl.clone() {
            Some(f) => f(&mut (*self.data).borrow()[self.range], address),
            None => panic!("can't read u16 to this storage"),
        }
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        match self.write_u8_impl.clone() {
            Some(f) => f(&mut (*self.data).borrow_mut()[self.range], address, value),
            None => panic!("can't write u8 to this storage"),
        }
    }

    fn write_u16(&mut self, address: u16, value: u16) {
        match self.write_u16_impl.clone() {
            Some(f) => f(&mut (*self.data).borrow_mut()[self.range], address, value),
            None => panic!("can't write u16 to this storage"),
        }
    }
}

enum Loc {
    RegA,
    RegB,
    RegC,
    RegD,
    RegE,
    RegH,
    RegL,
    RegAF,
    RegBC,
    RegDE,
    RegHL,
    RegSP,
    RegPC,
    Flags,
    RegIME,
    Mem(u16),
}

pub struct CPU {
    regs16: Storage,
    regs8: Storage,
    ram: Storage,
    rom: Storage,
    stack: Storage,
    io_ports: Storage,
    interrupt_master_enable: Storage,
    interrupt_enable: Storage,
}

impl CPU {
    fn get_storage(&mut self, loc: Loc) -> Storage {
        match loc {
            Loc::RegA => self.regs8.view(1..2),
            Loc::Flags => self.regs8.view(0..1),
            Loc::RegB => self.regs8.view(3..4),
            Loc::RegC => self.regs8.view(2..3),
            Loc::RegD => self.regs8.view(5..6),
            Loc::RegE => self.regs8.view(4..5),
            Loc::RegH => self.regs8.view(7..8),
            Loc::RegL => self.regs8.view(6..7),
            Loc::RegAF => self.regs16.view(0..2),
            Loc::RegBC => self.regs16.view(2..4),
            Loc::RegDE => self.regs16.view(4..6),
            Loc::RegHL => self.regs16.view(6..8),
            Loc::RegSP => self.regs16.view(8..10),
            Loc::RegPC => self.regs16.view(10..12),
            Loc::RegIME => self.interrupt_master_enable.view(0..1),
            Loc::Mem(address) => match address {
                0x0000..=0x7FFF => self.rom.view(address as usize..0x8000),
                0xC000..=0xDFFF => self.rom.view((address as usize - 0xC000)..0x2000),
                0xFF00..=0xFF4B => self.io_ports.view((address as usize - 0xFF00)..0x004C),
                0xFF80..=0xFFFE => self.stack.view((address as usize - 0xFF80)..0x007F),
                0xFFFF => self.interrupt_enable.view(0..1),
                _ => panic!("address 0x{:X} not mapped", address),
            },
        }
    }
}
