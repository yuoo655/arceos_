use axhal::mem::{phys_to_virt, virt_to_phys};
use core::ptr::{read_volatile, write_volatile};
const  REG_SDMA_DMA_CH_REMAP0:usize=	0x03000154;
const  REG_SDMA_DMA_CH_REMAP1:usize=	0x03000158;

const TABLE_SIZE:usize=8;

struct DmaRemapItem{
    hs_id:usize,
    channel_id:u8,
}

const REMAP_TABLE:[DmaRemapItem;TABLE_SIZE]=[
    DmaRemapItem{
        hs_id:13,
        channel_id:1
    },
    DmaRemapItem{
        hs_id:12,
        channel_id:0
    },
    DmaRemapItem{
        hs_id:7,
        channel_id:2
    },
    DmaRemapItem{
        hs_id:0,
        channel_id:3
    },
    DmaRemapItem{
        hs_id:20,
        channel_id:4
    },
    DmaRemapItem{
        hs_id:21,
        channel_id:5
    },
    DmaRemapItem{
        hs_id:2,
        channel_id:6
    },
    DmaRemapItem{
        hs_id:38,
        channel_id:7
    },
];

pub fn dma_hs_remap_init()
{
    let mut remap0_val=0;
    let mut remap1_val=0;
    for i in 0..TABLE_SIZE{
        let hs_id=REMAP_TABLE[i].hs_id;
        let mut channel_id=REMAP_TABLE[i].channel_id;
        if channel_id < 4{
            remap0_val |= hs_id << (channel_id << 3);
        } else {
            channel_id -= 4;
            remap1_val |= hs_id << (channel_id << 3);
        }
    }
    remap0_val |= 1 << 31;
	remap1_val |= 1 << 31;
    unsafe{
        write_volatile(phys_to_virt(REG_SDMA_DMA_CH_REMAP0.into()).as_usize() as *mut u32,remap0_val as u32);
        write_volatile(phys_to_virt(REG_SDMA_DMA_CH_REMAP1.into()).as_usize() as *mut u32,remap1_val as u32);
    }
}