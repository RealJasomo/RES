/**
 * Main Memory Bus for NES
 * Redirects memory accesses to the correct device
 */
use super::cpu::Cpu;
pub struct Bus{
    cpu: Cpu,
    ram:[u8; 64*1024]
}
impl Bus{
    pub fn new() -> Bus{
        Bus {
            cpu: Cpu::default(),
            ram: [0; 64*1024]
        }
    }
    pub fn write(addr: u16, data: u8){

    }
    pub fn read(addr: u16) -> u8{
        return 0;
    }
    pub fn debugPrint(&self) {
        println!("Cpu: {:?}\nPress anything to continue:",self.cpu);
        let mut pause = String::new();
        std::io::stdin().read_line(&mut pause);
        for i in 0..64*1024 {
            println!("Address:{:x?} : {:x?}", i, self.ram[i]);
        }
    }
}
