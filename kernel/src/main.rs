#![no_std]
#![no_main]
#![allow(clippy::similar_names)]
mod mem;
mod requests;
mod writer;

use crate::mem::heap::Allocator;
use core::arch::asm;
use core::fmt::Write;

use limine::request::{RequestsEndMarker, RequestsStartMarker};
use limine::BaseRevision;

use requests::FRAMEBUFFER_REQUEST;
use writer::buffer::{init_writer, FrameBufferWriter};

extern crate alloc;

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

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            // Init writer!
            init_writer(framebuffer);

            println!("Allocator initialized successfully");
            println!("Writer initialized correctly");
        }
    }

    hcf();
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    hcf();
}

fn hcf() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
