use std::{
    ffi::OsStr,
    io,
    process::{Child, Command, ExitStatus, Stdio},
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
/// Launches a sub process `cmd` using `args`
/// Stdout and stderr are piped and can then be retrieved using the Child returned
/// eg: child.stdout.take() and child.stderr.take()
pub fn spawn_process(cmd: &str, args: Vec<String>) -> Result<Child, ProcessError> {
    let child = Command::new(OsStr::new(&cmd))
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| ProcessError::SpawnProcessFail(err))?;

    Ok(child)
}
/// Polls the process status without blocking
/// Can be called in a loop as it will handling sleep so it doesn't use 100% of the cpu
/// This function may never return ProcessStatus::Done if it blocks waiting for stdin
/// In order to make sure we can actually kill a process that is blocked waiting for stdin,
/// call `stop_child` and then `capture_exit_status`
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

/// Kills Child
/// /!\ Currently some weird issues happen on windows where the child is not really killed
/// depending on the situation /!\
pub fn stop_child(child: &mut Child) -> Result<(), io::Error> {
    child.kill()
}
/// Captures the child exit status
/// This function will wait for the child process and drop the stdin pipe so if the child process
/// is blocked waiting for stdin, this will unblock it
pub fn capture_exit_status(child: &mut Child) -> Result<ExitStatus, io::Error> {
    child.wait()
}

/// Waits for the child process to end
/// While waiting, setting `should_stop` to true will kill the process
pub fn wait_child(
    child: &mut Child,
    should_stop: Arc<AtomicBool>,
) -> Result<ExitStatus, ProcessError> {
    loop {
        if should_stop.load(Ordering::Relaxed) {
            let _ = stop_child(child);
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
