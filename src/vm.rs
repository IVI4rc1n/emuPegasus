
struct Cpu {
    // CPU registers
    a: u8,
    x: u8,
    y: u8,
    sp: u16,
    pc: u16,
}

impl Cpu {
    fn print_regs(&self) {
        println!("Registers");
        println!("A: 0x{:x}", &self.a);
        println!("X: 0x{:x}", &self.x);
        println!("Y: 0x{:x}", &self.y);
        println!("SP: 0x{:x}", &self.sp);
        println!("PC: 0x{:x}", &self.pc);
    }
}

struct RAM {
    ram: Vec<u16>,
}

// impl RAM {
//     fn read_byte(&self, addr: &u16) -> u16 {
//         return &self.ram[addr];
//     }
// }

fn main() {
    let cpu = Cpu { a:0x0, x:0x0, y:0x0, sp:0x0100, pc:0x0 };
    let memory = RAM { ram:vec![0; 0x10000] };
    cpu.print_regs();
}
