#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(improper_ctypes)]

use std::mem::MaybeUninit;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[test]
fn test() {
    unsafe {
        let mut dev_list = MaybeUninit::<*mut *mut doca_devinfo>::uninit();
        let mut nb_dev = MaybeUninit::<u32>::uninit();

        let result = doca_devinfo_create_list(dev_list.as_mut_ptr(), nb_dev.as_mut_ptr());

        assert_eq!(result, 0);
    }
}
