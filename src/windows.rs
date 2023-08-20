//! Wrapper for readvm C function
use std::io::{Result, Error};
use std::ffi::*;
pub type Pid = *const std::ffi::c_void;

mod c {
    use std::ffi::*;
    use super::Pid;
    extern "system" {
        pub fn GetCurrentProcess() -> Pid;
        pub fn ReadProcessMemory(pid: Pid, src: *const c_void, dst: *mut c_void, size: usize, rsize: *mut usize) -> c_int;
    }
}

pub fn getpid() -> Pid {
    unsafe { c::GetCurrentProcess() }
}

pub unsafe fn peek_buf(pid: Pid, addr: usize, dst: *mut u8, size: usize) -> Result<usize> {
    let mut rsize = 0;
    match c::ReadProcessMemory(pid, addr as *const c_void, dst.cast::<c_void>(), size, &mut rsize) {
        0 => Err(Error::last_os_error()),
        _ => Ok(rsize),
    }
}

