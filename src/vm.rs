use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Vm {
    // CPU registers
    a: u8,
    x: u8,
    y: u8,
    sp: u16,
    pc: usize,
    ram: Ram,
}

impl Vm {

}

impl Vm {
    fn print_regs(&self) {
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
            0xa0 => self.exec_ldy_imm(arg),
            0xa2 => self.exec_ldx_imm(arg),
            0xa9 => self.exec_lda_imm(arg),
            _ => false,
        };
        return res;
    }

    fn exec_lda_imm(&mut self, arg: u8) -> bool {
        self.a = arg;
        self.pc += 2;
        return true;
    }

    fn exec_ldy_imm(&mut self, arg: u8) -> bool {
        self.y = arg;
        self.pc += 2;
        return true;
    }

    fn exec_ldx_imm(&mut self, arg: u8) -> bool {
        self.x = arg;
        self.pc += 2;
        return true;
    }

    fn run(&mut self) {
        println!("Runnig emulator...");
        while self.exec_opcode() {
            &self.print_regs();
        };

    }
}

pub struct Ram {
    ram: Vec<u8>,
}

impl Ram {
    fn read(&self, addr: usize) -> u8 {
        return self.ram[addr & 0xffff];
    }

    fn write(&mut self, addr: usize, val: u8) {
        self.ram[addr & 0xffff] = val;
    }

    fn load_to_ram(&mut self, addr: usize, data: Vec<u8>) {
        let size = data.len();
        for i in 0..size {
            self.ram[(addr & 0xffff) + i] = data[i];
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
    f.read_to_end(&mut buffer).expect("something went wrong reading the file");
    println!("File {} loaded", filename);

    // Initialize virtual machine
    let mut vm = Vm { a:0x0, x:0x0, y:0x0, sp:0x0100, pc:0xc00, ram: Ram{ ram: vec![0; 0x10000] }, };
    vm.ram.load_to_ram(0xc00, buffer);
    println!("Initial register values:");
    vm.print_regs();
    vm.run();
    vm.print_regs();
    println!("*** Emulator finished ***");
}
