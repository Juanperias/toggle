use limine::request::{BootloaderInfoRequest, FramebufferRequest};

#[used]
#[link_section = ".requests"]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[link_section = ".requests"]
pub static BOOT_INFO_REQUEST: BootloaderInfoRequest = BootloaderInfoRequest::new();
