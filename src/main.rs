use std::{ffi::CStr, ptr};

use libc::{
    c_char, c_int, c_void, MAP_SHARED, O_CREAT, O_RDWR, PROT_READ, PROT_WRITE, S_IRUSR, S_IWUSR,
    S_IXUSR,
};

#[cfg(unix)]
use std::os::unix::io::RawFd;

extern "C" {
    #[allow(dead_code)]
    fn atomic_load64(addr: *mut c_void) -> u64;
    #[allow(dead_code)]
    fn atomic_store64(addr: *mut c_void, val: u64) -> u64;

    fn shm_open(path: *const c_char, flags: c_int, mode: c_int) -> RawFd;
    fn shm_unlink(path: *const c_char) -> c_int;
    fn ftruncate(fd: RawFd, len: usize) -> c_int;
    fn mmap(
        addr: *mut c_void,
        len: usize,
        prot: c_int,
        flags: c_int,
        fd: RawFd,
        offset: usize,
    ) -> *mut c_void;
}

fn main() {
    let fd: RawFd = unsafe {
        shm_open(
            "/test\0".as_ptr() as *const _,
            O_CREAT | O_RDWR,
            (S_IRUSR | S_IWUSR | S_IXUSR) as c_int,
        )
    };
    println!("fd: {fd}");

    let rs = unsafe { ftruncate(fd, 2048) };
    println!("ftruncate: {rs}");

    let buf: *mut i8 = unsafe {
        mmap(
            ptr::null_mut(),
            1024,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            fd,
            0,
        )
    } as *mut _;
    println!("buffer address: {:?}", buf);

    let mut n = 0;
    while n == 0 {
        n = unsafe { *buf };
    }

    let msg = unsafe { CStr::from_ptr(buf.add(1)) };
    println!("{:?}", msg);

    let rs = unsafe { shm_unlink("/test\0".as_ptr() as *const _) };
    println!("shm_unlink: {rs}");
}
