#![no_std]
#![no_main]

mod uefi;
mod console;

#[no_mangle]
extern "win64" fn efi_main(_efi_image_handle: &usize, system_table: *const uefi::SystemTable) -> ! {
    uefi::initialize(system_table);
    console::reset();

    console::putc('H');
    console::putc('E');
    console::putc('L');
    console::putc('L');
    console::putc('O');
    console::putc('\n');
    console::putc('\r');
    console::putc('W');
    console::putc('O');
    console::putc('R');
    console::putc('L');
    console::putc('D');
    console::putc('!');
    
    unsafe { core::arch::asm!("MOV EAX, 0xFFDD"); }
    loop {}
}



use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { core::arch::asm!("MOV EAX, 0xBadDeed"); }
    loop {}
}


