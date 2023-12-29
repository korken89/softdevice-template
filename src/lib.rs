//! Definitions for use with the softdevice.

#![no_std]

/// Main API. Holds the vectors in flash and safe API to those.
#[repr(C)]
pub struct Softdevice {
    test: unsafe extern "C" fn() -> i32,
}

impl Softdevice {
    /// Create the softdevice from function pointers.
    pub const fn from_pointers(test: unsafe extern "C" fn() -> i32) -> Self {
        Self { test }
    }

    /// Create the softdevice from symbols defined in the linker script.
    pub fn from_linkerfile() -> Self {
        extern "C" {
            static __softdevice: Softdevice;
        }

        // Safety: If the linker variable exists, it is assumed that the user has added it
        //         according to requirements. Else there will be a linker error.
        unsafe {
            let start = &__softdevice as *const Softdevice;

            core::ptr::read(start)
        }
    }
}
