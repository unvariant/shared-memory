use std::sync::atomic::{
    AtomicUsize,
    Ordering,
};

use libc::{
    O_RDONLY, O_RDWR, O_CREAT, O_EXCL, O_TRUNC,
    S_IRUSR, S_IWUSR, S_IXUSR, S_IRGRP, S_IWGRP, S_IXGRP, S_IROTH, S_IWOTH, S_IXOTH,
    c_void, c_char, c_int,
    PROT_READ, PROT_WRITE, PROT_EXEC, PROT_NONE,
    MAP_SHARED,
};

#[cfg(unix)]
use std::os::unix::io::{
    RawFd,
    FromRawFd,
};

extern "C" {
    fn atomic_load64 (addr: * mut c_void) -> u64;
    fn atomic_store64 (addr: * mut c_void, val: u64) -> u64;

    fn shm_open (path: * const c_char, flags: c_int, mode: c_int) -> RawFd;
    fn shm_unlink (path: * const c_char) -> c_int;
    fn ftruncate (fd: RawFd, len: usize) -> c_int;
    fn mmap (addr: * mut c_void, len: usize, prot: c_int, flags: c_int, fd: RawFd, offset: usize) -> * mut c_void;
}

fn main () {
    let fd: RawFd = unsafe { shm_open ("/test\0".as_ptr() as * const _, O_CREAT | O_RDWR, (S_IRUSR | S_IWUSR | S_IXUSR | S_IRGRP | S_IWGRP | S_IXGRP | S_IROTH | S_IWOTH | S_IXOTH) as c_int) };
    println! ("fd: {fd}");

    let rs = unsafe { ftruncate (fd, 2048) };
    println! ("result: {rs}");

    let buf = unsafe { mmap (0 as * mut _, 1024, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0) };
    println! ("buf: {:?}", buf);

    let mut n = 0;
    unsafe { atomic_store64(buf, n) };

    while n == 0 {
        n = unsafe { atomic_load64(buf) };
    }

    println! ("n: {n}");

    let rs = unsafe { shm_unlink ("/test\0".as_ptr() as * const _) };
    println! ("result: {rs}");
}