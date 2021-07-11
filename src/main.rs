mod cpu;
mod instruction;
mod storage;

use std::{cell::RefCell, rc::Rc};

enum Flags {
    Zero = 4,
    HalfCarry,
    Subtraction,
    Carry,
}

struct CPU {
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
    fn get_storage(&mut self, loc: Loc) -> (&mut Storage, u16) {
        match loc {
            Loc::RegA => (&mut self.regs8, 1),
            Loc::Flags => (&mut self.regs8, 0),
            Loc::RegB => (&mut self.regs8, 3),
            Loc::RegC => (&mut self.regs8, 2),
            Loc::RegD => (&mut self.regs8, 5),
            Loc::RegE => (&mut self.regs8, 4),
            Loc::RegH => (&mut self.regs8, 7),
            Loc::RegL => (&mut self.regs8, 6),
            Loc::RegAF => (&mut self.regs16, 0),
            Loc::RegBC => (&mut self.regs16, 2),
            Loc::RegDE => (&mut self.regs16, 4),
            Loc::RegHL => (&mut self.regs16, 6),
            Loc::RegSP => (&mut self.regs16, 8),
            Loc::RegPC => (&mut self.regs16, 10),
            Loc::RegIME => (&mut self.interrupt_master_enable, 0),
            Loc::Mem(address) => match address {
                0x0000..=0x7FFF => (&mut self.rom, address),
                0xC000..=0xDFFF => (&mut self.ram, address - 0xC000),
                0xFF00..=0xFF4B => (&mut self.io_ports, 0),
                0xFF80..=0xFFFE => (&mut self.stack, address - 0xFF80),
                0xFFFF => (&mut self.interrupt_enable, 0),
                _ => panic!("address 0x{:X} not mapped", address),
            },
        }
    }

    fn push(&mut self, value: u16) {
        let sp = self.get_u16(Loc::RegSP) - 2;
        self.set_u16(Loc::Mem(sp), value);
        self.set_u16(Loc::RegSP, sp);
    }

    fn pop(&mut self) -> u16 {
        let sp = self.get_u16(Loc::RegSP);
        self.set_u16(Loc::RegSP, sp + 2);
        self.get_u16(Loc::Mem(sp))
    }

    fn set_u8(&mut self, loc: Loc, value: u8) {
        let (storage, address) = self.get_storage(loc);
        storage.write_8(address, value);
    }

    fn set_u16(&mut self, loc: Loc, value: u16) {
        let (storage, address) = self.get_storage(loc);
        storage.write_16(address, value);
    }

    fn get_u8(&mut self, loc: Loc) -> u8 {
        let (storage, address) = self.get_storage(loc);
        storage.read_8(address)
    }

    fn get_u16(&mut self, loc: Loc) -> u16 {
        let (storage, address) = self.get_storage(loc);
        storage.read_16(address)
    }

    fn get_flag(&mut self, flag: Flags) -> bool {
        self.get_u8(Loc::Flags) & (flag as u8) == 0
    }

    fn set_flag(&mut self, flag: Flags, value: bool) {
        let mask = (value as u8) << (flag as u8);
        let flags = self.get_u8(Loc::Flags);
        let unset = flags & !mask;
        let set = unset | mask;
        self.set_u8(Loc::Flags, set);
    }

    fn new(rom_file: &str) -> Self {
        let mut rom = std::fs::read(rom_file).expect("failed to load file");
        rom.resize(0x8000, 0);
        let regs = Rc::new(RefCell::new(vec![0; 12]));
        let mut this = Self {
            regs8: Storage::from_data(regs.clone(), StorageAccess::RW, StorageDataSize::Eight),
            regs16: Storage::from_data(regs, StorageAccess::RW, StorageDataSize::Sixteen),
            ram: Storage::new(0x2000, StorageAccess::RW, StorageDataSize::Any),
            rom: Storage::from_data(
                Rc::new(RefCell::new(rom)),
                StorageAccess::R,
                StorageDataSize::Any,
            ),
            stack: Storage::new(0x7E, StorageAccess::RW, StorageDataSize::Any),
            io_ports: Storage::new(0x4C, StorageAccess::RW, StorageDataSize::Eight),
            interrupt_master_enable: Storage::new(1, StorageAccess::W, StorageDataSize::Eight),
            interrupt_enable: Storage::new(1, StorageAccess::RW, StorageDataSize::Eight),
        };
        this.set_u16(Loc::RegAF, 0x01B0);
        this.set_u16(Loc::RegBC, 0x0013);
        this.set_u16(Loc::RegDE, 0x00D8);
        this.set_u16(Loc::RegHL, 0x014D);
        this.set_u16(Loc::RegPC, 0x0100);
        this.set_u16(Loc::RegSP, 0xFFFE);
        this
    }

    fn register_instructions() {}

    fn execute(&mut self) {
        let pc = self.get_u16(Loc::RegPC);
        let offset: i8 = match self.get_u8(Loc::Mem(pc)) {
            0x00 => 1,
            0x01 => {
                // LD BC, d16
                let d16 = self.get_u16(Loc::Mem(pc + 1));
                self.set_u16(Loc::RegBC, d16);
                3
            }
            0x18 => self.get_u8(Loc::Mem(pc + 1)) as i8,
            0x20 => {
                if self.get_flag(Flags::Zero) {
                    2
                } else {
                    self.get_u8(Loc::Mem(pc + 1)) as i8
                }
            }
            0x28 => {
                if self.get_flag(Flags::Zero) {
                    self.get_u8(Loc::Mem(pc + 1)) as i8
                } else {
                    2
                }
            }
            0x21 => {
                // LD HL, d16
                let d16 = self.get_u16(Loc::Mem(pc + 1));
                self.set_u16(Loc::RegHL, d16);
                3
            }
            0x31 => {
                // LD SP, d16
                let d16 = self.get_u16(Loc::Mem(pc + 1));
                self.set_u16(Loc::RegSP, d16);
                3
            }
            0x36 => {
                // LD (HL), d8
                let d8 = self.get_u8(Loc::Mem(pc + 1));
                let address = self.get_u16(Loc::RegHL);
                self.set_u8(Loc::Mem(address), d8);
                1
            }
            0x3E => {
                // LD A, d8
                let imm8 = self.get_u8(Loc::Mem(pc + 1));
                self.set_u8(Loc::RegA, imm8);
                2
            }
            0x47 => {
                // LD B, A
                let value = self.get_u8(Loc::RegA);
                self.set_u8(Loc::RegB, value);
                1
            }
            0x78 => {
                // LD A, B
                let value = self.get_u8(Loc::RegB);
                self.set_u8(Loc::RegA, value);
                1
            }
            0xAF => {
                // XOR A, A
                self.set_u8(Loc::RegA, 0);
                self.set_u8(Loc::Flags, 0);
                1
            }
            0xC3 => {
                let imm16 = self.get_u16(Loc::Mem(pc + 1));
                self.set_u16(Loc::RegPC, imm16);
                0
            }
            0xC9 => {
                // RET
                let pc = self.pop();
                self.set_u16(Loc::RegPC, pc);
                0
            }
            0xCD => {
                // CALL a16
                self.push(pc + 3);
                let imm16 = self.get_u16(Loc::Mem(pc + 1));
                self.set_u16(Loc::RegPC, imm16);
                0
            }
            0xE0 => {
                // LD (0xFF00 + a8), A
                let address = 0xFF00 | (self.get_u8(Loc::Mem(pc + 1)) as u16);
                let value = self.get_u8(Loc::RegA);
                self.set_u8(Loc::Mem(address), value);
                2
            }
            0xE6 => {
                // AND d8
                let acc = self.get_u8(Loc::RegA);
                let d8 = self.get_u8(Loc::Mem(pc + 1));
                let value = acc & d8;
                self.set_u8(Loc::RegA, value);
                self.set_u8(Loc::Flags, 0);
                self.set_flag(Flags::Zero, value == 0);
                self.set_flag(Flags::HalfCarry, true);
                2
            }
            0xEA => {
                let imm16 = self.get_u16(Loc::Mem(pc + 1));
                let value = self.get_u8(Loc::RegA);
                self.set_u8(Loc::Mem(imm16), value);
                3
            }
            0xF0 => {
                // LD A, (0xFF00 + a8)
                let address = 0xFF00 | (self.get_u8(Loc::Mem(pc + 1)) as u16);
                let value = self.get_u8(Loc::Mem(address));
                self.set_u8(Loc::RegA, value);
                2
            }
            0xF3 => {
                // DI: reset interrupt master enable (IME)
                self.set_u8(Loc::RegIME, 0);
                1
            }
            0xFE => {
                let imm8 = self.get_u8(Loc::Mem(pc + 1));
                let a = self.get_u8(Loc::RegA);
                self.set_flag(Flags::Zero, a == imm8);
                2
            }
            0xCB => match self.get_u8(Loc::Mem(pc + 1)) {
                0x87 => {
                    // RES 0, A
                    let a = self.get_u8(Loc::RegA);
                    self.set_u8(Loc::RegA, a & 0b11111110);
                    2
                }
                unknown_code => panic!("unhandled multi-byte opcode: 0xCB{:X}", unknown_code),
            },
            unknown_code => panic!("unhandled opcode: {:X}", unknown_code),
        };
        let new_pc = (self.get_u16(Loc::RegPC) as i32) + (offset as i32);
        if new_pc < 0 || new_pc > (u16::MAX as i32) {
            panic!("new pc is oob: 0x{:X}", new_pc);
        }
        self.set_u16(Loc::RegPC, new_pc as u16);
    }
}

fn main() {
    let rom = std::fs::read("rom/Pokemon Red.gb").expect("failed to load file");
    cpu::CPU::new(rom).run();
}
