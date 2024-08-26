use std::{
    ffi::OsStr,
    io,
    process::{Child, Command, ExitStatus},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

pub enum ProcessError {
    WaitChildFail,
    SpawnProcessFail(io::Error),
    Quit,
}
pub fn spawn_process(cmd: &String, args: Vec<String>) -> Result<Child, ProcessError> {
    let child = Command::new(OsStr::new(&cmd))
        .args(args)
        .spawn()
        .map_err(|err| ProcessError::SpawnProcessFail(err))?;

    Ok(child)
}
pub fn wait_child(
    child: &mut Child,
    should_stop: Arc<AtomicBool>,
) -> Result<ExitStatus, ProcessError> {
    loop {
        if should_stop.load(Ordering::Relaxed) {
            return Err(ProcessError::Quit);
        }
        match child.try_wait() {
            Ok(status) => match status {
                Some(exit_status) => return Ok(exit_status),
                None => sleep(Duration::from_millis(10)), //wait till its over,
            },
            Err(_) => return Err(ProcessError::WaitChildFail),
        };
    }
}
