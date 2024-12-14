use crate::sys::tss::TSS;
use lazy_static::lazy_static;
use x86_64::{
    instructions::{
        segmentation::{Segment, CS, DS, ES, SS},
        tables::load_tss,
    },
    structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
};
lazy_static! {
    pub static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let kernel_code = gdt.append(Descriptor::kernel_code_segment());
        let kernel_data = gdt.append(Descriptor::kernel_data_segment());
        let tss = gdt.append(Descriptor::tss_segment(&TSS));
        let user_code = gdt.append(Descriptor::user_code_segment());
        let user_data = gdt.append(Descriptor::user_data_segment());

        (
            gdt,
            Selectors {
                kernel_code,
                kernel_data,
                user_code,
                user_data,
                tss,
            },
        )
    };
}

pub fn init_gdt() {
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.kernel_code);
        SS::set_reg(GDT.1.kernel_data);
        DS::set_reg(GDT.1.kernel_data);
        ES::set_reg(GDT.1.kernel_data);

        load_tss(GDT.1.tss);
    }
}

pub struct Selectors {
    pub kernel_code: SegmentSelector,
    pub kernel_data: SegmentSelector,
    pub user_code: SegmentSelector,
    pub user_data: SegmentSelector,
    pub tss: SegmentSelector,
}
