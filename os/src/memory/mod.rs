mod frame_allocator;
pub mod paging;
pub mod memory_set;

use frame_allocator::SEGMENT_TREE_ALLOCATOR as FRAME_ALLOCATOR;
use riscv::addr::{
    VirtAddr,
    PhysAddr,
    Page,
    Frame
};
use crate::DYNAMIC_ALLOCATOR;
use crate::consts::*;
use memory_set::{
    MemorySet,
    attr::MemoryAttr,
    handler::Linear
};

pub fn alloc_frame() -> Option<Frame> {
    Some(Frame::of_ppn(FRAME_ALLOCATOR.lock().alloc()))
}

pub fn dealloc_frame(f: Frame) {
    FRAME_ALLOCATOR.lock().dealloc(f.number())
}

fn init_heap() {
    static mut HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
    unsafe {
        DYNAMIC_ALLOCATOR
            .lock()
            .init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

pub fn init(l: usize, r: usize) {
    FRAME_ALLOCATOR.lock().init(l, r);
    init_heap();

	kernel_remap();

    println!("++++ setup memory!    ++++");
}

pub fn access_pa_via_va(pa: usize) -> usize {
    pa + PHYSICAL_MEMORY_OFFSET
}

pub fn kernel_remap() {
    let mut memory_set = MemorySet::new();
    
    extern "C" {
        fn bootstack();
        fn bootstacktop();
    }
    memory_set.push(
        bootstack as usize,
        bootstacktop as usize,
        MemoryAttr::new(),
        Linear::new(PHYSICAL_MEMORY_OFFSET),
	None,
    );

    unsafe {
        memory_set.activate();
    }
}