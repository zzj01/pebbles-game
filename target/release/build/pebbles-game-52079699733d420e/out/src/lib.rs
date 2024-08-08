#![no_std]
#[allow(unused_imports)]
pub use orig_project::*;

#[allow(improper_ctypes)]
mod fake_gsys {
    extern "C" {
        pub fn gr_reply(
            payload: *const u8,
            len: u32,
            value: *const u128,
            err_mid: *mut [u8; 36],
        );
    }
}

#[no_mangle]
extern "C" fn metahash() {
    const METAHASH: [u8; 32] = [77, 31, 50, 94, 105, 192, 110, 236, 219, 185, 103, 73, 226, 166, 227, 137, 31, 31, 255, 81, 237, 141, 3, 173, 56, 71, 100, 53, 139, 196, 235, 214];
    let mut res: [u8; 36] = [0; 36];
    unsafe {
        fake_gsys::gr_reply(
            METAHASH.as_ptr(),
            METAHASH.len() as _,
            u32::MAX as _,
            &mut res as _,
        );
    }
}
