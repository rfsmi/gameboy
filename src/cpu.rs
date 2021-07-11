use super::storage::Storage;
use super::storage::StorageMode;

pub enum Loc {
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
    pub fn new(mut rom: Vec<u8>) -> Self {
        if rom.len() > 0x8000 {
            panic!("rom too large (0x{:X} > 0x8000)", rom.len());
        }
        rom.resize(0x8000, 0);
        let regs8 = Storage::new(12, StorageMode::RW, StorageMode::None);
        let mut this = Self {
            regs16: regs8.duplicate(StorageMode::None, StorageMode::RW),
            regs8,
            ram: Storage::new(0x2000, StorageMode::RW, StorageMode::RW),
            rom: Storage::from_data(rom, StorageMode::RW, StorageMode::RW),
            stack: Storage::new(0x7E, StorageMode::RW, StorageMode::RW),
            io_ports: Storage::new(0x4C, StorageMode::RW, StorageMode::RW),
            interrupt_master_enable: Storage::new(1, StorageMode::RW, StorageMode::None),
            interrupt_enable: Storage::new(1, StorageMode::RW, StorageMode::None),
        };
        this.get_storage(Loc::RegAF).write_u16(0x01B0);
        this.get_storage(Loc::RegBC).write_u16(0x0013);
        this.get_storage(Loc::RegDE).write_u16(0x00D8);
        this.get_storage(Loc::RegHL).write_u16(0x014D);
        this.get_storage(Loc::RegPC).write_u16(0x0100);
        this.get_storage(Loc::RegSP).write_u16(0xFFFE);
        this
    }

    fn get_storage(&mut self, loc: Loc) -> Storage {
        match loc {
            Loc::RegA => self.regs8.view(1),
            Loc::Flags => self.regs8.view(0),
            Loc::RegB => self.regs8.view(3),
            Loc::RegC => self.regs8.view(2),
            Loc::RegD => self.regs8.view(5),
            Loc::RegE => self.regs8.view(4),
            Loc::RegH => self.regs8.view(7),
            Loc::RegL => self.regs8.view(6),
            Loc::RegAF => self.regs16.view(0),
            Loc::RegBC => self.regs16.view(2),
            Loc::RegDE => self.regs16.view(4),
            Loc::RegHL => self.regs16.view(6),
            Loc::RegSP => self.regs16.view(8),
            Loc::RegPC => self.regs16.view(10),
            Loc::RegIME => self.interrupt_master_enable.view(0),
            Loc::Mem(address) => match address {
                0x0000..=0x7FFF => self.rom.view(address),
                0xC000..=0xDFFF => self.rom.view(address - 0xC000),
                0xFF00..=0xFF4B => self.io_ports.view(address - 0xFF00),
                0xFF80..=0xFFFE => self.stack.view(address - 0xFF80),
                0xFFFF => self.interrupt_enable.view(0),
                _ => panic!("address 0x{:X} not mapped", address),
            },
        }
    }

    fn run(&mut self) {
        loop {}
    }
}
