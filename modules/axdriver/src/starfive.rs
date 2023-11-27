use axalloc::global_allocator;
use axhal::{mem::{phys_to_virt, virt_to_phys}, arch::flush_tlb};
use core::{alloc::Layout, ptr::NonNull};
use driver_net::starfive::StarfiveHal;
use axhal::time::{busy_wait, Duration};

pub struct StarfiveHalImpl;

impl StarfiveHal for StarfiveHalImpl {
    fn dma_alloc_pages(pages: usize) -> (usize, usize) {
        let vaddr = if let Ok(vaddr) = global_allocator().alloc_pages(pages, 0x1000) {
            vaddr
        } else {
            panic!("RxRing alloc_pages failed");
        };
        let paddr = virt_to_phys(vaddr.into()).as_usize();

        // info!("dma_alloc_pages vaddr:{:x} paddr:{:x}", vaddr, paddr);
        (vaddr, paddr)
    }

    fn dma_free_pages(vaddr: usize, pages: usize) {
        global_allocator().dealloc_pages(vaddr, pages);
    }


    fn phys_to_virt(pa: usize) -> usize {
        // info!("phys_to_virt pa:{:x}", pa);
        let va = phys_to_virt(pa.into()).as_usize();
        va
    }
    fn virt_to_phys(va: usize) -> usize {
        let pa = virt_to_phys(va.into()).as_usize();
        pa
    }

    fn mdelay(_m_times:usize)
    {
        busy_wait(Duration::from_millis(_m_times.try_into().unwrap()));
    }

    fn fence() {
        
    }
}
