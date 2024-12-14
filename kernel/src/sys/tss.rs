use core::ptr::addr_of;
use lazy_static::lazy_static;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const IST_STACK_SIZE: usize = 4096 * 5;

static mut BSP_IST_STACK: [u8; IST_STACK_SIZE] = [0; IST_STACK_SIZE];
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    pub static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            let stack_start = VirtAddr::from_ptr(addr_of!(BSP_IST_STACK));
            stack_start + IST_STACK_SIZE as u64
        };
        tss
    };
}
