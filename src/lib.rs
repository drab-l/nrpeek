//! Peek process info
use std::io::Result;
use std::mem::MaybeUninit;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(target_os = "windows", path = "windows.rs")]
mod os;
use os::Pid;

pub struct Peek {
    pid: Pid,
}
#[allow(unused_macros)]
macro_rules! LINE { () => { eprintln!("{}", line!()) } }

pub fn getpid() -> Pid {
    os::getpid()
}

unsafe fn peek_buf(pid: Pid, addr: usize, dst: *mut u8, size: usize) -> Result<usize> {
    os::peek_buf(pid, addr, dst, size)
}

impl Peek {
    pub fn new(pid: Pid) -> Self {
        Peek{pid}
    }

    /// Peek specfied type's data from target process
    /// # Arguments
    /// * `pid` - A peek target process ID
    /// * `addr` - A peek target address
    pub fn peek_data<T>(&self, addr: usize) -> Result<T> {
        let pid = self.pid;
        let size = std::mem::size_of::<T>();
        let mut buf = MaybeUninit::<T>::uninit();
        unsafe { peek_buf(pid, addr, buf.as_mut_ptr().cast::<u8>(), size)?; }
        Ok(unsafe { buf.assume_init() })
    }

    /// Peek null terminated string
    /// # Arguments
    /// * `pid` - A peek target process ID
    /// * `addr` - A peek target address
    pub fn peek_until_null(&self, addr: usize) -> Result<Vec<u8>> {
        let pid = self.pid;
        let mut addr = addr;
        let mut res = vec![];
        loop {
            const PEEK_SIZE: usize = 32;
            let mut buf = Vec::<u8>::with_capacity(PEEK_SIZE);
            unsafe {
                let len = peek_buf(pid, addr, buf.as_mut_ptr(), buf.capacity())?;
                buf.set_len(len);
            }
            let error = buf.len() != buf.capacity();
            addr += buf.len();
            if let Some(len) = buf.iter().position(|x| *x == 0) {
                buf.truncate(len);
                res.append(&mut buf);
                break
            } else {
                res.append(&mut buf);
            }
            if error { break; }
        }
        Ok(res)
    }

    /// Peek data from target process to `Vec<u8>`
    /// # Arguments
    /// * `pid` - A peek target process ID
    /// * `addr` - A peek target address
    /// * `dst` - Destination vector that capacity shall be allocated.
    pub fn peek_vec(&self,  addr: usize, dst:&mut Vec<u8>) -> Result<()> {
        let pid = self.pid;
        let size = dst.capacity();
        unsafe {
            let ptr = dst.as_mut_ptr();
            peek_buf(pid, addr, ptr, size)?;
            dst.set_len(size);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_peek_vec() {
        let peek = Peek::new(getpid());
        let src = vec![1,2,3,4,5,6,7,8,9,10,11,12];
        let mut dst = Vec::<u8>::with_capacity(12);
        peek.peek_vec(src.as_ptr() as usize, &mut dst).unwrap();
        assert_eq!(src, dst);
    }
    #[test]
    fn test_peek_until_null() {
        let peek = Peek::new(getpid());
        let src = vec![1,2,3,4,5,6,7,8,9,0,11,12];
        let mut dst = peek.peek_until_null(src.as_ptr() as usize).unwrap();
        dst.append(&mut vec![0,11,12]);
        assert_eq!(src, dst);
    }
    #[test]
    fn test_peek_data() {
        #[derive(Debug, PartialEq)]
        struct Test {
            a: u32, b: u32
        }
        let peek = Peek::new(getpid());
        let src = Test{ a: 1, b: 2 };
        let dst = peek.peek_data::<Test>(std::ptr::addr_of!(src) as usize).unwrap();
        assert_eq!(src, dst);
    }
}
