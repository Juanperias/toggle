use alloc::collections::VecDeque;
use x86_64::{
    registers::control::Cr3,
    structures::paging::{
        FrameAllocator, FrameDeallocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags,
        PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};
use lazy_static::lazy_static;
use spin::Mutex;

const KERNEL_MEM_OFFSET: u64 = 0xFFFF_8000_0000_0000;


const FOUR_KIB: u64 = 4096;

lazy_static! {
    pub static ref MemMapper: Mutex<(PhysAlloc, OffsetPageTable<'static>)> =
        Mutex::new((PhysAlloc::new(0x1000), get_mapper()));
}

#[derive(Clone)]
pub struct PhysAlloc {
    pointer: u64,
    free_list: VecDeque<PhysFrame<Size4KiB>>,
}

impl PhysAlloc {
    pub fn new(pointer: u64) -> Self {
        PhysAlloc {
            pointer,
            free_list: VecDeque::new(),
        }
    }
}

unsafe impl FrameAllocator<Size4KiB> for PhysAlloc {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        if let Some(frame) = self.free_list.pop_front() {
            return Some(frame);
        }

        let addr = PhysAddr::new(self.pointer);
        let frame = PhysFrame::containing_address(addr);

        self.pointer += FOUR_KIB;
        Some(frame)
    }
}

impl FrameDeallocator<Size4KiB> for PhysAlloc {
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame<Size4KiB>) {
        self.free_list.push_front(frame);
    }
}

pub fn map_addr(addr: u64) {
    let mut mem_mapper_guard = MemMapper.try_lock().unwrap();

    let page: Page<Size4KiB> = Page::containing_address(VirtAddr::new(addr));
    let frame: PhysFrame<Size4KiB> = mem_mapper_guard
        .0
        .allocate_frame()
        .expect("Cannot allocate more memory");
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    let (allocator, mapper) = &mut *mem_mapper_guard;

    unsafe {
        mapper
            .map_to(page, frame, flags, allocator)
            .expect("Cannot map addr")
            .flush();
    }
}

fn get_mapper() -> OffsetPageTable<'static> {
    let addr = VirtAddr::new(KERNEL_MEM_OFFSET);
    unsafe { OffsetPageTable::new(get_page_table(addr), addr) }
}

fn get_page_table(virt_addr: VirtAddr) -> &'static mut PageTable {
    let (frame, _) = Cr3::read();
    let phys_addr = frame.start_address().as_u64();
    let virt = virt_addr.as_u64() + phys_addr;
    let addr = unsafe { &mut *(virt as *mut PageTable) };
    addr
}
