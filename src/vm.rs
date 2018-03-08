
struct Vm {
    a: u8,
    x: u8,
    y: u8,
    sp: u16,
    pc: u16
}

impl Vm {
    fn print_regs(&self) {
        println!("Registers");
        println!("A: {}", &self.a);
        println!("X: {}", &self.a);
        println!("Y: {}", &self.a);
        println!("SP: {}", &self.a);
        println!("PC: {}", &self.a);
    }
}

fn main() {
    let emu = Vm { a:0, x:0, y:0, sp:0, pc:0 };
    emu.print_regs();
}
