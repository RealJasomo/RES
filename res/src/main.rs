/**
 * Rust Entertainment System
 * Author: Jason Cramer
 */
mod bus;
mod cpu;

use bus::Bus;

fn main() {
    let bus = Bus::new();
    bus.debug_print();
}
