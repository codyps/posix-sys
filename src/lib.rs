extern crate libc;

extern "C" {
    pub fn pread(fd: libc::c_int, buf: *mut libc::c_void, len: libc::size_t, offs: libc::off_t) -> libc::ssize_t;
    pub fn pwrite(fd: libc::c_int, buf: *const libc::c_void, len: libc::size_t, offs: libc::off_t) -> libc::ssize_t;
}
