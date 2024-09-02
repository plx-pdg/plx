use std::sync::{
    atomic::AtomicBool,
    mpsc::{self, Sender},
    Arc,
};

use crate::{
    core::{
        runner::runner::{RunEvent, Runner},
        work::{work::Work, work_type::WorkType},
    },
    models::{event::Event, exo::Exo},
};

use super::compiler::Compiler;
pub struct CompileRunner {
    runner: Runner,
}
impl CompileRunner {
    pub fn new(compiler: &Compiler, exo: &Exo, output_path: &std::path::PathBuf) -> Option<Self> {
        let cmd = compiler.cmd();
        let mut args = compiler.args(&exo.files);
        match output_path.to_str() {
            Some(path) => {
                args.push(String::from("-o"));
                args.push(String::from(path));
            }
            None => return None,
        }
        Some(Self {
            runner: Runner::new(String::from(cmd), args),
        })
    }
    pub fn get_full_command(&self) -> String {
        self.runner.get_full_command()
    }
}
impl Work for CompileRunner {
    fn run(&self, tx: Sender<Event>, stop: Arc<AtomicBool>) -> bool {
        let (runner_tx, runner_rx) = mpsc::channel();
        let _ = self.runner.run(runner_tx, stop);
        while let Ok(msg) = runner_rx.recv() {
            let send = match msg {
                RunEvent::ProcessCreationFailed(_) => {
                    return false;
                }
                RunEvent::ProcessCreated => tx.send(Event::CompilationStart),

                RunEvent::ProcessEnd(success) => tx.send(Event::CompilationEnd(success)),

                RunEvent::ProcessNewOutputLine(line) => tx.send(Event::CompilationOutputLine(line)),
            };
            if send.is_err() {
                break;
            }
        }
        return true;
    }

    fn work_type(&self) -> WorkType {
        WorkType::Compilation
    }
}

#[cfg(test)]
mod test {

    use std::{path::PathBuf, time::Duration};

    use crate::core::{file_utils::file_utils::list_dir_files, parser::from_dir::FromDir};

    use super::*;
    fn build_exo(path: &std::path::PathBuf) -> Exo {
        Exo::from_dir(path)
            .expect(&format!(
                "Couldn't build exo from {}",
                path.to_str().unwrap()
            ))
            .0
    }
    fn create_compiler(
        compiler: &Compiler,
        exo_path: &PathBuf,
        output_path: &PathBuf,
    ) -> CompileRunner {
        assert!(!output_path.exists());
        let exo = build_exo(&exo_path);
        CompileRunner::new(compiler, &exo, &output_path).expect("Couldn't create compile runner")
    }
    fn compile(compiler: CompileRunner, output_path: &PathBuf) {
        let (tx, rx) = mpsc::channel();
        let stop = Arc::new(AtomicBool::new(false));
        compiler.run(tx, stop);
        match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(Event::CompilationEnd(success)) => assert!(success),
            Ok(event) => println!("Event {:#?}", event),
            Err(err) => println!("Err {:#?}", err),
        }
        //This helps out work out possible problems
        let output_folder = output_path.parent().unwrap().to_path_buf();
        println!("{:#?}", list_dir_files(&output_folder));
        assert!(output_path.exists());
        std::fs::remove_file(output_path).expect("Couldn't remove file");
    }
    #[test]
    fn compile_valid_exo_one_file() {
        let path = PathBuf::from("examples")
            .join("mock-plx-project")
            .join("intro")
            .join("basic-args");
        let output_path = PathBuf::from("target").join("compile_valid_exo_one_file");
        let compiler = create_compiler(&Compiler::Gcc, &path, &output_path);

        let command = compiler.get_full_command();

        assert!(command.contains("gcc"));
        assert!(command.contains("main.c"));
        assert!(command.contains(&format!("-o {}", output_path.to_str().unwrap())));

        compile(compiler, &output_path);
    }

    #[test]
    fn compile_valid_exo_multiple_file() {
        let path = PathBuf::from("examples")
            .join("mock-plx-project")
            .join("datastructures")
            .join("queue");
        let output_path = PathBuf::from("target").join("queue");
        let compiler = create_compiler(&Compiler::Gcc, &path, &output_path);

        let command = compiler.get_full_command();

        println!("Command: {}", command);
        assert!(command.contains("gcc"));
        assert!(command.contains("main.c"));
        assert!(command.contains("queue.c"));
        assert!(command.contains(&format!("-o {}", output_path.to_str().unwrap())));
        compile(compiler, &output_path);
    }
}
