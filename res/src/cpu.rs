/**
 * 6502 CPU for NES
 * Processes OPcodes and responsible for all arithmetic operations
 */

/**
 * Status Register:
 * Bit 0: Carry  P & 0x01 
 * Bit 1: Zero  P & 0x02
 * Bit 2: Interrupt Disable P & 0x04
 * Bit 3: Decimal P & 0x08
 * Bit 4,5: B flag P & 0x30
 * Bit 6: Overflow P & 0x40
 * Bit 7: Negative P & 0x80
 */
#[derive(Default, Debug)]
pub struct Cpu{
    a: u8, // Accumulator
    x: u8, // Index
    y: u8, // Index
    pc: u16, // Program Counter
    sp: u8, // Stack Pointer
    p: u8  // Status Register NVxxDIZC
}
impl Cpu{
}