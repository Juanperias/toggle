use crate::println;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::arch::asm;
use core::fmt::Write;

#[derive(Debug)]
pub struct CpuInfo {
    pub vendor: String,
    pub features: Vec<CpuFeature>,
}

impl CpuInfo {
    pub fn new() -> Self {
        Self {
            vendor: get_vendor(),
            features: CpuFeature::check(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CpuFeature {
    Apic,
    Vmx,
}

impl CpuFeature {
    pub fn check() -> Vec<Self> {
        let mut features = Vec::new();
        for i in Self::iter() {
            if i.is_supported() {
                features.push(i);
            }
        }

        features
    }
    fn iter() -> impl Iterator<Item = CpuFeature> {
        [CpuFeature::Apic, CpuFeature::Vmx].iter().copied()
    }
    fn is_supported(&self) -> bool {
        let mut edx: u32 = 0;
        let mut ecx: u32 = 0;
        unsafe {
            asm!("mov eax, 1", "cpuid", out("edx") edx, out("ecx") ecx);
        }

        match self {
            Self::Apic => edx & (1 << 9) != 0,
            Self::Vmx => ecx & (1 << 5) != 0,
        }
    }
}

fn get_vendor() -> String {
    let mut vendor = [0u32; 3];

    unsafe {
        asm!(
            "xor eax, eax",
            "cpuid",
            "mov {0:e}, ebx",
            "mov {1:e}, edx",
            "mov {2:e}, ecx",
            out(reg) vendor[0],
            out(reg) vendor[1],
            out(reg) vendor[2],
            out("eax") _, out("ecx") _, out("edx") _,
        );
    }

    let bytes: &[u8; 12] = unsafe { core::mem::transmute(&vendor) };
    String::from_utf8_lossy(bytes).to_string()
}
