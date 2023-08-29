// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::vec;

// Import CKB syscalls and structures
// https://docs.rs/ckb-std/
use ckb_std::{
    ckb_constants::Source,
    debug,
    env::argv,
    syscalls::{self, current_cycles, spawn},
};
use core::ffi::CStr;

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let argvs = argv();
    debug!(
        "current cycle:{}, argv length:{} - 1",
        current_cycles(),
        argvs.len()
    );
    if argvs.len() > 0 {
        debug!(
            "current cycle:{}, argv length:{} - 2 ",
            current_cycles(),
            argvs.len()
        );
    }
    let mut exit_code: i8 = 0;
    let mut content: [u8; 10] = [0; 10];

    let content_length: u64 = content.len() as u64;
    let mut spawn_args = syscalls::SpawnArgs {
        memory_limit: 8,
        exit_code: &mut exit_code as *mut i8,
        content: content.as_mut_ptr(),
        content_length: &content_length as *const u64 as *mut u64,
    };
    let cstr1 = CStr::from_bytes_with_nul(b"arg0\0").unwrap();
    let mut cstrs = vec![cstr1];
    for _i in 0..argvs.len() {
        cstrs.push(cstr1);
    }
    spawn_args.memory_limit = 3;
    spawn(0, Source::CellDep, 0, cstrs.as_slice(), &spawn_args);
    debug!(
        "current cycle:{}, argv length:{} - 3",
        current_cycles(),
        argvs.len()
    );

    Ok(())
}
