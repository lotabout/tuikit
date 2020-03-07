use std::error::Error;
use std::os::unix::io::RawFd;
use std::time::Duration;

use nix::sys::select;
use nix::sys::time::{TimeVal, TimeValLike};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn duration_to_timeval(duration: Duration) -> TimeVal {
    let sec = duration.as_secs() * 1000 + (duration.subsec_millis() as u64);
    TimeVal::milliseconds(sec as i64)
}

pub fn wait_until_ready(fd: RawFd, signal_fd: Option<RawFd>, timeout: Duration) -> Result<()> {
    let mut timeout_spec = if timeout == Duration::new(0, 0) {
        None
    } else {
        Some(duration_to_timeval(timeout))
    };

    let mut fdset = select::FdSet::new();
    fdset.insert(fd);
    signal_fd.map(|fd| fdset.insert(fd));
    let n = select::select(None, &mut fdset, None, None, &mut timeout_spec)?;

    if n < 1 {
        Err("select return file descriptor other than 1".into())
    } else if fdset.contains(fd) {
        Ok(())
    } else {
        Err("interrupted when waiting for signal".into())
    }
}
