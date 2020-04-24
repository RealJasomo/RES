/**
 * Main Memory Bus for NES
 * Redirects memory accesses to the correct device
 */
use super::cpu::Cpu;
pub struct Bus{
    cpu: Cpu,
    ram: [u8; 64*1024] //64kb of ram
}
impl Bus{
    pub fn new() -> Bus{
        Bus {
            cpu: Cpu::default(),
            ram: [0; 64*1024]
        }
    }
    pub fn write(&mut self, addr: u16, data: u8){
        if addr >= 0x0000 && addr <= 0xFFFF{
            self.ram[addr as usize] = data;
        }
    }
    pub fn read(&self,addr: u16) -> u8{
        return self.ram[addr as usize];
    }
    pub fn debug_print(&self) {
        println!("Cpu: {:?}\nPress anything to continue:",self.cpu);
        let mut pause = String::new();
        std::io::stdin().read_line(&mut pause);
        for i in 0..64*1024 {
            println!("Address:{:x?} : {:x?}", i, self.ram[i]);
        }
    }
}
