use nix::sys::select;
use nix::sys::time::{TimeVal, TimeValLike};
use std::error::Error;
use std::os::unix::io::RawFd;
use std::time::Duration;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn duration_to_timeval(duration: Duration) -> TimeVal {
    let sec = duration.as_secs() * 1000 + (duration.subsec_millis() as u64);
    TimeVal::milliseconds(sec as i64)
}

pub fn wait_until_ready(fd: RawFd, timeout: Duration) -> Result<()> {
    if timeout == Duration::new(0, 0) {
        return Ok(());
    }

    let mut fdset = select::FdSet::new();
    fdset.insert(fd);
    let n = select::select(
        None,
        &mut fdset,
        None,
        None,
        &mut duration_to_timeval(timeout),
    )?;
    if n == 1 {
        Ok(())
    } else {
        Err("select return file descriptor other than 1".into())
    }
}
