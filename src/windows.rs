//! Wrapper for readvm C function
use std::io::{Result, Error};
use std::ffi::*;
pub type Pid = u32;
type HDL = *const std::ffi::c_void;
type HWND = *const std::ffi::c_void;
pub struct HANDLE {
    hdl: HDL
}

impl Drop for HANDLE {
    fn drop(&mut self) {
        unsafe { c::CloseHandle(self.hdl) };
    }
}
mod c {
    use std::ffi::*;
    use super::*;
    extern "system" {
        pub fn GetCurrentProcessId() -> Pid;
        pub fn GetCurrentProcess() -> HDL;
        pub fn OpenProcess(access: u32, inherit: i32, pid: Pid) -> HDL;
        pub fn CloseHandle(hdl: HDL) -> i32;
        pub fn ReadProcessMemory(hdl: HDL, src: *const c_void, dst: *mut c_void, size: usize, rsize: *mut usize) -> c_int;
        pub fn FindWindowA(class: *const c_char, window: *const c_char) -> HWND;
        pub fn GetWindowThreadProcessId(window: HWND, pid: *mut Pid) -> Pid;
    }
    pub const PROCESS_VM_READ: u32 = 0x10;
}

pub fn get_current_handle() -> HANDLE {
    HANDLE { hdl: unsafe { c::GetCurrentProcess() } }
}

pub fn get_current_id() -> Pid {
    unsafe { c::GetCurrentProcessId() }
}

pub fn get_handle_by_window_name(name: &str) -> Result<HANDLE> {
    let hwnd = unsafe { c::FindWindowA(std::ptr::null(), name.as_ptr().cast::<c_char>()) };
    if hwnd.is_null() {
        return Err(Error::last_os_error())
    }
    let mut pid = 0;
    let tid = unsafe { c::GetWindowThreadProcessId(hwnd, &mut pid) };
    if tid == 0 {
        return Err(Error::last_os_error())
    }
    pid_to_handle(pid)
}

pub fn pid_to_handle(pid: Pid) -> Result<HANDLE> {
    let hdl = unsafe { c::OpenProcess(c::PROCESS_VM_READ, 0, pid) };
    if hdl.is_null() {
        Err(Error::last_os_error())
    } else {
        Ok(HANDLE { hdl })
    }
}

pub unsafe fn peek_buf(hdl: &HANDLE, addr: usize, dst: *mut u8, size: usize) -> Result<usize> {
    let mut rsize = 0;
    match c::ReadProcessMemory(hdl.hdl, addr as *const c_void, dst.cast::<c_void>(), size, &mut rsize) {
        0 => Err(Error::last_os_error()),
        _ => Ok(rsize),
    }
}

