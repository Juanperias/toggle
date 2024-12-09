#![no_std]
#![no_main]
#![allow(clippy::similar_names)]
#![feature(abi_x86_interrupt)]
mod mem;
mod requests;
mod sys;
mod writer;

use crate::mem::heap::Allocator;
use core::fmt::Write;

use crate::sys::idt::init_idt;
use alloc::boxed::Box;
use alloc::format;
use limine::request::{RequestsEndMarker, RequestsStartMarker};
use limine::BaseRevision;

use requests::FRAMEBUFFER_REQUEST;
use writer::buffer::{init_writer, FrameBufferWriter};
extern crate alloc;
use crate::sys::gdt::init_gdt;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator::new();

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".requests_start_marker"]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[link_section = ".requests_end_marker"]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[no_mangle]
extern "C" fn main() -> ! {
    assert!(BASE_REVISION.is_supported());

    // INIT MEMORY ALLOCATOR!
    ALLOCATOR.init();

    init_writer();

    init_idt();
    init_gdt();

    println!("Allocator initialized successfully");
    println!("Writer initialized successfully");
    println!("Idt initialized successfully");
    println!("Gdt initialized successfully");

    loop {}
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    println!("{:?}", info);

    loop {}
}
