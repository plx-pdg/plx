use std::{
    io::{BufRead, BufReader, Read},
    process::ExitStatus,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
        Arc,
    },
    thread::{self, sleep, JoinHandle},
    time::Duration,
};

use crate::core::process::process_handler::{self, ProcessStatus};

#[derive(Debug, PartialEq)]
pub enum RunEvent {
    ProcessCreationFailed(String),
    ProcessCreated,
    ProcessEnd(bool),
    ProcessNewOutputLine(String),
}
pub struct Runner {
    command: String,
    args: Vec<String>,
}

impl Runner {
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self { command, args }
    }
    pub fn get_full_command(&self) -> String {
        format!("{} {}", &self.command, &self.args.join(" "))
    }

    fn read_stream<T: Read>(tx: Sender<RunEvent>, stream: T) {
        let reader = BufReader::new(stream);
        reader.lines().for_each(|line| match line {
            Ok(line) => {
                let _ = tx.send(RunEvent::ProcessNewOutputLine(line));
            }
            Err(_) => return,
        });
    }
    fn launch_stream_reader<T>(tx: Sender<RunEvent>, stream: T) -> JoinHandle<()>
    where
        T: Read + Send + 'static,
    {
        thread::spawn(move || Runner::read_stream(tx, stream))
    }

    pub fn run(
        &self,
        tx: Sender<RunEvent>,
        should_stop: Arc<AtomicBool>,
    ) -> Result<ExitStatus, ()> {
        let mut process = process_handler::spawn_process(&self.command, self.args.clone())
            .map_err(|err| {
                let _ = tx.send(RunEvent::ProcessCreationFailed(format!("{:?}", err)));
            })?;

        let _ = tx.send(RunEvent::ProcessCreated);
        let mut stdout_thread = {
            if let Some(stdout) = process.stdout.take() {
                Some(Runner::launch_stream_reader(tx.clone(), stdout))
            } else {
                None
            }
        };
        let mut stderr_thread = {
            if let Some(stderr) = process.stderr.take() {
                Some(Runner::launch_stream_reader(tx.clone(), stderr))
            } else {
                None
            }
        };

        let exit_status = loop {
            if should_stop.load(Ordering::Relaxed) {
                if process_handler::stop_child(&mut process).is_err() {
                    //Couldn't kill child process
                    eprintln!("Couldn't kill child process");
                    break None;
                }
                break process_handler::capture_exit_status(&mut process).ok();
            }
            match process_handler::get_process_status(&mut process) {
                Err(_) => break None,
                Ok(ProcessStatus::Done(status)) => break Some(status),
                Ok(ProcessStatus::Running) => {
                    sleep(Duration::from_millis(100));
                }
            };
        };

        if let Some(t) = stdout_thread.take() {
            let _ = t.join();
        }

        if let Some(t) = stderr_thread.take() {
            let _ = t.join();
        }

        let success = if let Some(exit_status) = exit_status {
            exit_status.success()
        } else {
            false
        };

        tx.send(RunEvent::ProcessEnd(success));
        exit_status.ok_or(())
    }
}

#[cfg(test)]
mod test {
    use std::{
        process::Command,
        sync::mpsc::{channel, Receiver},
    };

    use ntest::timeout;

    use super::*;

    fn compile_program(c_file: &str, target: &str) {
        Command::new("gcc")
            .arg(c_file)
            .arg("-o")
            .arg(target)
            .output()
            .expect("Couldn't compile program");
    }
    fn launch_program(
        target: &str,
        stop: Arc<AtomicBool>,
    ) -> (JoinHandle<Result<ExitStatus, ()>>, Receiver<RunEvent>) {
        let runner = Runner::new(target.to_string(), vec![]);

        let (tx, rx) = channel();
        let thread_stop = stop.clone();

        (
            thread::spawn(move || {
                let exit = runner.run(tx, thread_stop);
                assert!(exit.is_ok());
                exit
            }),
            rx,
        )
    }
    fn run_blocking_program(target: &str) {
        sleep(Duration::from_secs(1));
        let stop = Arc::new(AtomicBool::new(false));
        let (handler, _) = launch_program(target, stop.clone());
        // Stop should kill the process no matter the condition it is in
        stop.store(true, Ordering::Relaxed);
        handler
            .join()
            .expect("Couldn't join thread")
            .expect("Couldn't get child exit status");
    }
    fn compile_and_run_blocking_program(c_file: &str, target: &str) {
        compile_program(c_file, target);
        run_blocking_program(target);
        let _ = std::fs::remove_file(target);
    }
    #[test]
    #[timeout(5000)]
    fn test_stuck_stdin() {
        if cfg!(windows) {
            return;
        };
        // This code blocks reading stdin forever
        let c_file = "./examples/basics/c/wait_stdin.c";
        let target = "./target/wait_stdin";
        compile_and_run_blocking_program(c_file, target);
    }

    #[test]
    #[timeout(5000)]
    fn test_infinite_loop() {
        if cfg!(windows) {
            return;
        };
        // This code does while(1)
        let c_file = "./examples/basics/c/infinite_loop.c";
        let target = "./target/infinite_loop";
        compile_and_run_blocking_program(c_file, target);
    }

    #[test]
    #[timeout(5000)]
    fn test_infinite_loop_with_sig_mapped() {
        if cfg!(windows) {
            return;
        };
        // This code does while(1) and ignores sigterm and sigint
        let c_file = "./examples/basics/c/infinite_loop_map_signals.c";
        let target = "./target/infinit_loop_map_signals";
        compile_and_run_blocking_program(c_file, target);
    }

    #[test]
    #[timeout(10000)]
    fn test_stdout_during_run() {
        if cfg!(windows) {
            return;
        };
        //This code loops forever and prints Hello <i> every second
        let c_file = "./examples/basics/c/infinite_loop.c";
        let target = "./target/infinite_loop_stdout";
        compile_program(c_file, target);

        let stop = Arc::new(AtomicBool::new(false));
        let (handler, rx) = launch_program(target, stop.clone());

        //give the program some time to start
        sleep(Duration::from_millis(1000));

        assert_eq!(
            rx.recv().expect("Didn't receive data from process"),
            RunEvent::ProcessCreated
        );
        for i in 1..=4 {
            assert_eq!(
                rx.recv().expect("Didn't receive data from process"),
                RunEvent::ProcessNewOutputLine(String::from(format!("Hello {}", i)))
            );
        }
        stop.store(true, Ordering::Relaxed);
        handler
            .join()
            .expect("Couldn't join thread")
            .expect("Couldn't get child exit status");
        let _ = std::fs::remove_file(target);
    }
}
