use x86_64::instructions::hlt;

pub fn hcf() -> ! {
    loop {
        hlt();
    }
}
