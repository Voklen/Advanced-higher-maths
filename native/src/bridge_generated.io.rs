use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_square(port_: i64, n: u32) {
    wire_square_impl(port_, n)
}

#[no_mangle]
pub extern "C" fn wire_combinatoric(port_: i64) {
    wire_combinatoric_impl(port_)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

// Section: wire structs

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}