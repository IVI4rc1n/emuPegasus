use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Vm {
    // CPU registers
    a: u8,
    x: u8,
    y: u8,
    sp: u16,
    pc: u16,
    // RAM memory
    ram: Ram,
}

impl Vm {
    fn run(&mut self) {
        println!("Runnig emulator...");
        while self.exec_opcode() {
            &self.print_regs();
        }
        println!("*** Emulator finished ***");
    }

    fn print_regs(&self) {
        // Print registers values
        println!("A: 0x{:02x}", &self.a);
        println!("X: 0x{:02x}", &self.x);
        println!("Y: 0x{:02x}", &self.y);
        println!("SP: 0x{:04x}", &self.sp);
        println!("PC: 0x{:04x}", &self.pc);
    }

    fn exec_opcode(&mut self) -> bool {
        let opcode = self.ram.read(self.pc);
        let arg = self.ram.read(self.pc + 1);

        let res = match opcode {
            0xA0 | 0xA2 | 0xA9 => self.exec_ld_reg_imm(opcode, arg),
            0x84 | 0x85 | 0x86 => self.exec_st_reg_zp(opcode, arg),
            0xA4 | 0xA5 | 0xA6 => self.exec_ld_reg_zp(opcode, arg),
            0xB4 | 0xB5 => self.exec_ld_reg_zpx(opcode, arg),
            0xAC | 0xAD | 0xAE => self.exec_ld_reg_abs(opcode, arg),
            0xBC | 0xBD => self.exec_ld_reg_abs_x(opcode, arg),
            _ => false,
        };
        return res;
    }

    //
    // MOS6052 Assembly instructions:
    //

    // LDY, LDX, LDA - immediate
    fn exec_ld_reg_imm(&mut self, opcode: u8, arg: u8) -> bool {
        match opcode {
            0xA0 => self.y = arg,
            0xA2 => self.x = arg,
            0xA9 => self.a = arg,
            _ => return false,
        }
        self.pc += 2;
        return true;
    }

    // LDY, LDX, LDA - zero page
    fn exec_ld_reg_zp(&mut self, opcode: u8, arg: u8) -> bool {
        match opcode {
            0xA4 => self.y = self.ram.read(arg as u16),
            0xA6 => self.x = self.ram.read(arg as u16),
            0xA5 => self.a = self.ram.read(arg as u16),
            _ => return false,
        }
        self.pc += 2;
        return true;
    }

    // LDY, LDA - zero page index with X
    fn exec_ld_reg_zpx(&mut self, opcode: u8, arg: u8) -> bool {
        match opcode {
            0xB4 => self.y = self.ram.read((arg + self.x) as u16),
            0xB5 => self.a = self.ram.read((arg + self.x) as u16),
            _ => return false,
        }
        self.pc += 2;
        return true;
    }

    // LDY, LDX, LDA - absolute
    fn exec_ld_reg_abs(&mut self, opcode: u8, arg: u8) -> bool {
        let arg2: u16 = self.ram.read(self.pc + 2) as u16;
        let addr = (arg as u16) | (arg2 << 8);
        match opcode {
            0xAC => self.y = self.ram.read(addr),
            0xAE => self.x = self.ram.read(addr),
            0xAD => self.a = self.ram.read(addr),
            _ => return false,
        }
        self.pc += 3;
        return true;
    }

    // LDY, LDA - absolute indexed with X
    fn exec_ld_reg_abs_x(&mut self, opcode: u8, arg: u8) -> bool {
        match opcode {
            0xBC => self.y = self.ram.read((arg + self.x) as u16),
            0xBD => self.a = self.ram.read((arg + self.x) as u16),
            _ => return false,
        }
        self.pc += 2;
        return true;
    }

    // STY, STX, STA - zero page
    fn exec_st_reg_zp(&mut self, opcode: u8, arg: u8) -> bool {
        match opcode {
            0x84 => self.ram.write(arg as u16, self.y),
            0x85 => self.ram.write(arg as u16, self.a),
            0x86 => self.ram.write(arg as u16, self.x),
            _ => return false,
        }
        self.pc += 2;
        return true;
    }
}

pub struct Ram {
    ram: Vec<u8>,
}

impl Ram {
    // Read 1 byte
    fn read(&self, addr: u16) -> u8 {
        return self.ram[addr as usize];
    }

    // Write 1 byte
    fn write(&mut self, addr: u16, val: u8) {
        self.ram[addr as usize] = val;
    }

    // Load input file
    fn load_data(&mut self, addr: u16, data: Vec<u8>) {
        let size = data.len();
        for i in 0..size {
            self.ram[addr as usize + i] = data[i];
        }
    }
}

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: emuPegasus <filename>");
        return;
    }
    let filename = &args[1];
    // Open file
    println!("Opening file {}...", filename);
    let mut f = File::open(filename).expect("file not found");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("something went wrong while reading the file");
    println!("File {} loaded", filename);

    // Initialize virtual machine
    let mut vm = Vm { a:0x0, x:0x0, y:0x0, sp:0x0100, pc:0xc00, ram: Ram{ ram: vec![0; 0x10000] }, };
    vm.ram.load_data(0xc00, buffer);
    println!("Initial register values:");
    vm.print_regs();
    // Run emulator
    vm.run();
}
