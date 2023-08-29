//! Wrapper for readvm C function
use std::io::{Result, Error};
pub type Pid = std::ffi::c_int;
pub type HANDLE = Pid;

mod c {
    use std::ffi::*;
    use super::Pid;
    extern "C" {
        pub fn getpid() -> Pid;
        pub fn process_vm_readv(pid: Pid, dst: *const iovec, dstcnt: c_ulong,
                                src: *const iovec, srccnt: c_ulong, flags: c_uint) -> isize;
    }

    #[repr(C)]#[allow(non_camel_case_types)]
    pub struct iovec {
        pub iov_base: *mut c_void,
        pub iov_len: usize,
    }
}

pub fn get_current_id() -> Pid {
    unsafe { c::getpid() }
}

pub fn get_current_handle() -> HANDLE {
    unsafe { c::getpid() }
}

pub fn pid_to_handle(pid: Pid) -> HANDLE {
    pid as HANDLE
}

pub unsafe fn peek_buf(hdl: &HANDLE, addr: usize, dst: *mut u8, size: usize) -> Result<usize> {
    let local = c::iovec{iov_base: dst.cast::<std::ffi::c_void>(), iov_len: size};
    let remote = c::iovec{iov_base: addr as *mut std::ffi::c_void, iov_len: size};
    match c::process_vm_readv(*hdl, &local, 1, &remote, 1, 0) {
        res if res < 0 => Err(Error::last_os_error()),
        res => Ok(res as usize),
    }
}

