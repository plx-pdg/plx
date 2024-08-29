use std::{
    ffi::OsStr,
    io,
    process::{Child, Command, ExitStatus, Output, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

#[derive(Debug)]
pub enum ProcessError {
    WaitChildFail,
    SpawnProcessFail(io::Error),
    Quit,
}
pub enum ProcessStatus {
    Done(ExitStatus),
    Running,
}
pub fn spawn_process(cmd: &str, args: Vec<String>) -> Result<Child, ProcessError> {
    let child = Command::new(OsStr::new(&cmd))
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| ProcessError::SpawnProcessFail(err))?;

    Ok(child)
}
pub fn get_process_status(child: &mut Child) -> Result<ProcessStatus, ProcessError> {
    match child.try_wait() {
        Ok(status) => match status {
            Some(exit_status) => Ok(ProcessStatus::Done(exit_status)),
            None => {
                sleep(Duration::from_millis(10));
                Ok(ProcessStatus::Running)
            }
        },
        Err(_) => return Err(ProcessError::WaitChildFail),
    }
}
pub fn wait_child(
    child: &mut Child,
    should_stop: Arc<AtomicBool>,
) -> Result<ExitStatus, ProcessError> {
    loop {
        if should_stop.load(Ordering::Relaxed) {
            return Err(ProcessError::Quit);
        }
        match get_process_status(child) {
            Ok(status) => match status {
                ProcessStatus::Done(exit_status) => return Ok(exit_status),
                ProcessStatus::Running => {
                    sleep(Duration::from_millis(10));
                }
            },
            Err(_) => {
                return Err(ProcessError::WaitChildFail);
            }
        }
    }
}
