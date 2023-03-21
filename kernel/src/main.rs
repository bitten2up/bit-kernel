#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader_api::{entry_point, BootInfo, FrameBufferInfo};
mod framebuffer;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo, info: FrameBufferInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        
        let mut value = 0x90;
        for byte in framebuffer.buffer_mut() {
            *byte = value;
            value = value.wrapping_add(1);
        }
    }
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}