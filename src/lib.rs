#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(improper_ctypes)]

use std::{ffi::c_void, mem::MaybeUninit};
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Allocate single element from buffer inventory and point it to the buffer
/// defined by `addr` & `len` arguments.
///
/// ## Details
/// Call `doca_buf_dec_refcount` to return the buffer to the inventory (until ref count == 0).
///
/// ## Arguments
/// * \[in\] `inventory` - The DOCA Buf inventory. MUST NOT BE NULL AND MUST BE STARTED.
/// * \[in\] `mmap` - DOCA memory map structure. MUST NOT BE NULL AND MUST BE STARTED.
/// * \[in\] `addr` - The start address of the data inside the buffer.
/// * \[in\] `len` - The length in bytes of the data.
/// * \[out\] `buf` - Doca buf allocated and initialized with args. MUST NOT BE NULL.
///
/// ## Returns
/// * `Ok(doca_buf)` - In case of success, returns the allocated DOCA buffer.
/// * `Err` - In case of failure:
///   - `DOCA_ERROR_NO_MEMORY` - if doca_buf_inventory is empty.
///
/// ## Safety
/// This function is unsafe because it deals with raw pointers and requires that
/// the inventory and mmap are properly initialized and not null.
pub unsafe fn doca_buf_inventory_buf_get_by_addr(
    inventory: *mut doca_buf_inventory,
    mmap: *mut doca_mmap,
    addr: *mut c_void,
    len: usize,
    buf: *mut *mut doca_buf,
) -> doca_error_t {
    unsafe { doca_buf_inventory_buf_get_by_args(inventory, mmap, addr, len, addr, 0, buf) }
}

/// Allocate single element from buffer inventory and point it to the buffer
/// defined by `data` & `data_len` arguments.
///
/// ## Details
/// Call doca_buf_dec_refcount to return the buffer to the inventory (until ref count == 0).
///
/// ## Arguments
/// \[in\] `inventory` - The DOCA Buf inventory. MUST NOT BE NULL AND MUST BE STARTED.
/// \[in\] `mmap` - DOCA memory map structure. MUST NOT BE NULL AND MUST BE STARTED.
/// \[in\] `data` - The start address of the data inside the buffer.
/// \[in\] `data_len` - The length in bytes of the data.
/// \[out\] `buf` - Doca buf allocated and initialized with args. MUST NOT BE NULL.
///
/// ## Returns
/// * `DOCA_SUCCESS` - in case of success.
/// * `doca_error` code - in case of failure:
///   - `DOCA_ERROR_NO_MEMORY` - if doca_buf_inventory is empty.
///
pub unsafe fn doca_buf_inventory_buf_get_by_data(
    inventory: *mut doca_buf_inventory,
    mmap: *mut doca_mmap,
    data: *mut c_void,
    data_len: usize,
    buf: *mut *mut doca_buf,
) -> doca_error_t {
    unsafe {
        doca_buf_inventory_buf_get_by_args(inventory, mmap, data, data_len, data, data_len, buf)
    }
}

#[test]
fn test() {
    unsafe {
        let mut dev_list = MaybeUninit::<*mut *mut doca_devinfo>::uninit();
        let mut nb_dev = MaybeUninit::<u32>::uninit();

        let result = doca_devinfo_create_list(dev_list.as_mut_ptr(), nb_dev.as_mut_ptr());

        assert_eq!(result, doca_error::DOCA_SUCCESS);
    }
}
