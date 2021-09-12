use lazy_static::lazy_static;
use x86_64::instructions::segmentation::{CS, Segment};
use x86_64::instructions::tables::load_tss;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    static ref TTS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        // TODO: allocate stack properly
        const STACK_SIZE: usize = 5 * 4096;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
        let stack_end = stack_start + STACK_SIZE;
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = stack_end;
        tss
    };
}

lazy_static! {
    static ref GDT_WITH_SELECTORS: GlobalDescriptorTableWithSelectors = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TTS));
        GlobalDescriptorTableWithSelectors  { gdt, selectors: Selectors { code_selector, tss_selector } }
    };
}

struct GlobalDescriptorTableWithSelectors {
    gdt: GlobalDescriptorTable,
    selectors: Selectors
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector
}

pub fn init() {
    GDT_WITH_SELECTORS.gdt.load();
    unsafe {
        CS::set_reg(GDT_WITH_SELECTORS.selectors.code_selector);
        load_tss(GDT_WITH_SELECTORS.selectors.tss_selector);
    }
}
