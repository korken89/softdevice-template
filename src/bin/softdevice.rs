#![no_std]
#![no_main]

use softdevice_template::Softdevice;

/// Test method, should be in the vector table.
pub extern "C" fn test() -> i32 {
    10
}

/// Vector-table definition, fill in all parts here.
#[link_section = ".vector_table.softdevice"]
#[no_mangle]
#[used]
pub static __SOFTDEVICE: Softdevice = Softdevice::from_pointers(test);

// A panic handler is required for compilation.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
