//! Definitions for use with the softdevice.

#![no_std]

/// Main API. Holds the vectors in flash and safe API to those.
#[repr(C)]
pub struct Softdevice {
    // TODO: Add API functions here.
    test: unsafe extern "C" fn() -> i32,
}

impl Softdevice {
    /// Create the softdevice from function pointers.
    pub const fn from_pointers(test: unsafe extern "C" fn() -> i32) -> Self {
        Self { test }
    }

    /// Create the softdevice from symbols defined in the linker script.
    #[inline]
    pub fn from_linkerfile() -> &'static Self {
        extern "C" {
            static __softdevice: Softdevice;
        }

        // Safety: If the linker variable exists, it is assumed that the user has added it
        //         according to requirements. Else there will be a linker error.
        unsafe { &__softdevice }
    }

    // TODO: Add safe API access methods here.
}
