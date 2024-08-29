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

use crate::{
    core::process::process_handler::{self, ProcessStatus},
    models::event::Event,
};

pub struct Runner {
    command: String,
    args: Vec<String>,
}

impl Runner {
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self { command, args }
    }

    fn read_stream<T: Read>(tx: Sender<Event>, stdout: T) {
        let reader = BufReader::new(stdout);
        reader.lines().for_each(|line| match line {
            Ok(line) => {
                let _ = tx.send(Event::ProcessOutputLine(line));
            }
            Err(_) => return,
        });
    }
    fn launch_stream_reader<T>(tx: Sender<Event>, stream: T) -> JoinHandle<()>
    where
        T: Read + Send + 'static,
    {
        thread::spawn(move || Runner::read_stream(tx, stream))
    }

    pub fn run(self, tx: Sender<Event>, should_stop: Arc<AtomicBool>) -> Result<ExitStatus, ()> {
        let mut process =
            process_handler::spawn_process(&self.command, self.args).map_err(|_err| {
                let _ = tx.send(Event::ProcessCreationFailed);
            })?;

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
        exit_status.ok_or(())
    }
}

#[cfg(test)]
mod test {
    use std::{process::Command, sync::mpsc::channel};

    use ntest::timeout;

    use super::*;

    fn compile_program(c_file: &str, file_name: &str) {
        Command::new("gcc")
            .arg(c_file)
            .arg("-o")
            .arg(file_name)
            .output()
            .expect("Couldn't compile program");
    }
    fn run_blocking_program(file_name: &str) {
        let runner = Runner::new(file_name.to_string(), vec![]);

        let (tx, _) = channel();
        let stop = Arc::new(AtomicBool::new(false));
        let thread_stop = stop.clone();

        let handler = thread::spawn(move || runner.run(tx, thread_stop));
        sleep(Duration::from_secs(1));

        // Stop should kill the process no matter the condition it is in
        stop.store(true, Ordering::Relaxed);

        handler
            .join()
            .expect("Couldn't join thread")
            .expect("Couldn't get child exit status");
    }
    fn compile_and_run_blocking_program(c_file: &str, file_name: &str) {
        compile_program(c_file, file_name);
        run_blocking_program(file_name);
        let _ = std::fs::remove_file(file_name);
    }
    #[test]
    #[timeout(2000)]
    fn test_stuck_stdin() {
        // This code blocks reading stdin forever
        let c_file = "./examples/basics/c/wait_stdin.c";
        let file_name = "./wait_stdin";
        compile_and_run_blocking_program(c_file, file_name);
    }

    #[test]
    #[timeout(2000)]
    fn test_infinite_loop() {
        // This code does while(1)
        let c_file = "./examples/basics/c/infinite_loop.c";
        let file_name = "./infinite_loop";
        compile_and_run_blocking_program(c_file, file_name);
    }

    #[test]
    #[timeout(2000)]
    fn test_infinite_loop_with_sig_mapped() {
        // This code does while(1)
        let c_file = "./examples/basics/c/infinite_loop_map_signals.c";
        let file_name = "./infinit_loop_map_signals";
        compile_and_run_blocking_program(c_file, file_name);
    }
}
