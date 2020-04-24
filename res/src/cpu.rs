/**
 * 6502 CPU for NES
 * Processes OPcodes and responsible for all arithmetic operations
 */
use super::bus::Bus;
/**
 * Status Register: & for read | for write
 * Bit 0: Carry  P & 0x01 
 * Bit 1: Zero  P & 0x02
 * Bit 2: Interrupt Disable P & 0x04
 * Bit 3: Decimal P & 0x08
 * Bit 4: B flag P & 0x10
 * Bit 5: unused
 * Bit 6: Overflow P & 0x40
 * Bit 7: Negative P & 0x80
 */
enum StatusRegister{
    C = 0x01,// carry
    Z = 0x02, // zero
    I = 0x04, // interupt
    D = 0x08, // decimal
    B = 0x10, // break
    V = 0x40, // overflow
    N = 0x80 // negative
}
#[derive(Default, Debug)]
pub struct Cpu{
    a: u8, // Accumulator
    x: u8, // Index
    y: u8, // Index
    pc: u16, // Program Counter
    sp: u8, // Stack Pointer
    p: u8,  // Status Register NVxxDIZC
    addr_rel: u16, //helper for relative addressing
    addr_abs: u16,  //helper for absolute addressing
    fetched: u8  //fetched value for multiple arguments
}
impl Cpu{
    fn clock(&mut self, bus : &mut Bus)
    {

    }
    //read from bus
    fn read(&mut self, bus: &mut Bus, addr:u16) -> u8{
        return bus.read(addr);
    }
    //Addressing modes input bus
    //implied
    fn imp(&mut self, bus: &mut Bus) -> u8{
        self.fetched = self.a;
        return 0;
    }
    //immediate
    fn imm(&mut self, bus: &mut Bus) -> u8{
        self.pc += 1;
        self.addr_abs = self.pc;
        return 0;
    }
    //zero page
    fn zp(&mut self, bus: &mut Bus) -> u8{
        self.addr_abs = self.read(bus,self.pc) as u16;	
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        return 0;
    }
    //zero page x
    fn zpx(&mut self, bus: &mut Bus) -> u8{
        self.addr_abs = self.read(bus,self.pc + self.x as u16) as u16;	
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        return 0;
    }
    //zero page y
    fn zpy(&mut self, bus: &mut Bus) -> u8{
        self.addr_abs = self.read(bus,self.pc + self.y as u16) as u16;	
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        return 0;
    }
    //relative
    fn rel(&mut self, bus: &mut Bus) -> u8{
        self.addr_rel = self.read(bus,self.pc) as u16;	
        self.pc += 1;
        if self.addr_rel & 0x0080 == 0x80 {
            self.addr_rel |= 0xFF00;
        }
        return 0;
    }
    //absolute
    fn abs(&mut self, bus: &mut Bus) -> u8{
        let lo = self.read(bus,self.pc);
        self.pc += 1;
        let hi = self.read(bus,self.pc);
        self.pc += 1;
        self.addr_abs = ((hi as u16 )<< 8)|(lo as u16);
        return 0;
    }
    //absolute x
    fn abx(&mut self, bus: &mut Bus) -> u8{
        let lo = self.read(bus,self.pc);
        self.pc += 1;
        let hi = self.read(bus,self.pc);
        self.pc += 1;
        self.addr_abs = ((hi as u16 )<< 8)|(lo as u16);
        self.addr_abs += self.x as u16;
        if (self.addr_abs & 0xFF00) != ((hi as u16)<< 8) {
            return 1;
        } else{
	    	return 0;
    }
    //TODO: indirect access modes
}
}