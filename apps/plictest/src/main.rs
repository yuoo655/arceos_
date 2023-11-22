#![no_std]
#![no_main]
extern crate axhal;
use axlog::*;
use core::ptr::{read_volatile, write_volatile};
use axhal::mem::phys_to_virt;

#[no_mangle]
fn main() {
    libax::println!("C906 PLIC Interrupt Test app, testing PLIC external interrupts on c906 platform!");    
    libax::println!("Step1: register an irq handler for external interrupt number 32");    
    libax::println!("Step2: write 1 to the interrupt bit of irqn32 to its Interrupt Pending register PLIC_IPx");    
    libax::println!("Step3: wait for the interrupt handling process to run!");    
    let plic_base = 0x7000_0000;
    let plic_ip0_offset = 0x000_1000;
    axhal::irq::register_handler_common(32, ||{
        info!("irq_num:32, interrupt handling process is running.");
    });

    unsafe{
        info!("Trying writes 1 to PLIC_IP register for irq_num:32 ");
        let plic_ip_va = phys_to_virt((plic_base + plic_ip0_offset + 4 * (32 / 32)).into()).as_usize() as *mut u32;
        let ori = read_volatile(plic_ip_va);
        write_volatile(plic_ip_va, ori | 1 << (32 % 32));
        info!("write interrupt pending register completes, waiting for interrupt handling process");
    }
}
