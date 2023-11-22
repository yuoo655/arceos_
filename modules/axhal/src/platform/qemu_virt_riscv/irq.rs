//! TODO: PLIC

use crate::irq::IrqHandler;
use lazy_init::LazyInit;
use riscv::register::sie;
use core::ptr::{read_volatile,write_volatile};
use crate::mem::{phys_to_virt, virt_to_phys};

/// `Interrupt` bit in `scause`
pub(super) const INTC_IRQ_BASE: usize = 1 << (usize::BITS - 1);

/// Supervisor software interrupt in `scause`
#[allow(unused)]
pub(super) const S_SOFT: usize = INTC_IRQ_BASE + 1;

/// Supervisor timer interrupt in `scause`
pub(super) const S_TIMER: usize = INTC_IRQ_BASE + 5;

/// Supervisor external interrupt in `scause`
pub(super) const S_EXT: usize = INTC_IRQ_BASE + 9;

static TIMER_HANDLER: LazyInit<IrqHandler> = LazyInit::new();

/// The maximum number of IRQs.
pub const MAX_IRQ_COUNT: usize = 128;

/// The timer IRQ number (supervisor timer interrupt in `scause`).
pub const TIMER_IRQ_NUM: usize = S_TIMER;

const PLIC_BASE:usize = 0x7000_0000;
const PLIC_PRIO1_OFFSET:usize   = 0x0;
const PLIC_IP0_OFFSET:usize     = 0x000_1000;
const PLIC_H0_SIE0_OFFSET:usize = 0x000_2080;
const PLIC_H0_STH_OFFSET:usize  = 0x020_1000;
const PLIC_SCLAIM_OFFSET:usize  = 0x020_1004;

macro_rules! with_cause {
    ($cause: expr, @TIMER => $timer_op: expr, @EXT => $ext_op: expr $(,)?) => {
        match $cause {
            S_TIMER => $timer_op,
            S_EXT => $ext_op,
            _ => panic!("invalid trap cause: {:#x}", $cause),
        }
    };
}

/// Enables or disables the given IRQ.
pub fn set_enable(irq_num: usize, _enabled: bool) {
    // if scause == S_EXT {
    // TODO: set enable in PLIC
    // let irqn = scause & !
    let x:u32 = (irq_num as u32) / 32;
    let y:u32 = (irq_num as u32) % 32;
    
    let plic_prio_reg_addr = PLIC_BASE + 4*irq_num as usize;
    let plic_h0_sie_reg_addr:usize = PLIC_BASE + PLIC_H0_SIE0_OFFSET + 4*x as usize;
    let plic_h0_sie_reg_enable_mask:u32 = 1 << y;

    // info!("Plic enable for irq_num:{}!\n",irq_num);
    unsafe{
        let plic_h0_sie_reg_va:*mut u32 = phys_to_virt(plic_h0_sie_reg_addr.into()).as_usize() as *mut u32;
        let plic_prio_reg_va:*mut u32 = phys_to_virt(plic_prio_reg_addr.into()).as_usize() as *mut u32;

        // info!("The plic_h0_sie_reg_va:{:#x}", phys_to_virt(plic_h0_sie_reg_addr.into()).as_usize());
        let sie_curr_reg_value = read_volatile(plic_h0_sie_reg_va);
        // info!("The sie_curr_reg_value is {:#x}", sie_curr_reg_value);

        if _enabled {
            let prio = read_volatile(plic_prio_reg_va);
            // setup external interupt source's priority.  irqn's priority, 0 means it will not be handlers.
            if prio != 7 {
                write_volatile(plic_prio_reg_va, 7); // c906 support 0-31
            }
            // enable external interupt for this irqn/interupt source
            write_volatile(plic_h0_sie_reg_va, sie_curr_reg_value | plic_h0_sie_reg_enable_mask);              
        }else{
            // disable interrupt for this irq_num
            write_volatile(plic_h0_sie_reg_va, sie_curr_reg_value & !plic_h0_sie_reg_enable_mask);     
        }

        // let sie_curr_reg_value = read_volatile(plic_h0_sie_reg_va);
        // info!("The sie_curr_reg_value after write volatile: {}", sie_curr_reg_value);
    }
    // remember to setup PLIC_TH reg during interrupt init!  set it to 0 to allow all the external interrupt sources / irqns.
    // }
}

/// Registers an handler for Timer interrupt only
///
/// It also enables the IRQ if the registration succeeds. It returns `false` if
/// the registration failed.
pub fn register_handler(scause: usize, handler: IrqHandler) -> bool {
    with_cause!(
        scause,
        @TIMER => if !TIMER_HANDLER.is_init() {
            TIMER_HANDLER.init_by(handler);
            true
        } else {
            false
        },
        @EXT => crate::irq::register_handler_common(scause & !INTC_IRQ_BASE, handler),
    )
}

/// Dispatches the IRQ.
///
/// This function is called by the common interrupt handler. It looks
/// up in the IRQ handler table and calls the corresponding handler. If
/// necessary, it also acknowledges the interrupt controller after handling.
pub fn dispatch_irq(scause: usize) {
    with_cause!(
        scause,
        @TIMER => {
            trace!("IRQ: timer");
            TIMER_HANDLER();
        },
        @EXT => {
            let plic_sclaim_va = phys_to_virt((PLIC_BASE + PLIC_SCLAIM_OFFSET).into()).as_usize() as *mut u32;
            let mut pending_int_irqn:u32 = 0;
            unsafe {
                // reading PLIC sclaim register to get the interrupt that is pending for handle
                pending_int_irqn = read_volatile(plic_sclaim_va);                
            }
            // 0 means no interrupt pending
            if pending_int_irqn != 0{                
                // sie::clear_sie(); // disable external int, 
                crate::irq::dispatch_irq_common(pending_int_irqn as usize); // TODO: get IRQ number from PLIC
                // sie::set_sie();
            }
            unsafe{
                // write back to sclaim register, indicating we have completed interrupt handling
                write_volatile(plic_sclaim_va,pending_int_irqn);   
            }
        }
    );
}

pub(super) fn init_percpu() {
    // enable soft interrupts, timer interrupts, and external interrupts
    unsafe {
        sie::set_ssoft();
        sie::set_stimer();
        sie::set_sext();
    }
    // disable all interrupts and set priority of all interrupts to 1
    #[cfg(feature = "irq")]
    for i in 1..MAX_IRQ_COUNT{
        set_enable(i, false); // disable all interrupts, enable them when register a handler for them
        plic_prio_init(i as u32, 1); // set priority of all interrupts to 1 in advance
    }
    // setup plic threshold to 0
    #[cfg(feature = "irq")]
    plic_threshold_setup(0);

}

fn plic_threshold_setup(threshold: u32){
    let plic_h0_sth_va = phys_to_virt((PLIC_BASE + PLIC_H0_STH_OFFSET).into()).as_usize() as *mut u32;
    // write 0 to plic_h0_sth to enable all the interrupts.
    unsafe{
        write_volatile(plic_h0_sth_va, threshold); 
    }
}

fn plic_prio_init(irq_num:u32, val:u32){
    let plic_prio_reg_addr = PLIC_BASE + 4*irq_num as usize;
    let plic_prio_reg_va:*mut u32 = phys_to_virt(plic_prio_reg_addr.into()).as_usize() as *mut u32;
    unsafe{write_volatile(plic_prio_reg_va, val); }
}