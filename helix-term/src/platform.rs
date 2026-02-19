/// Wrapper for `getsid` from Redox's relibc
/// The Relibc provides this function natively
#[cfg(target_os = "redox")]
pub unsafe extern "C" fn getsid(pid: libc::pid_t) -> libc::pid_t {
    extern "C" {
        fn getsid(pid: libc::pid_t) -> libc::pid_t;
    }
    getsid(pid)
}