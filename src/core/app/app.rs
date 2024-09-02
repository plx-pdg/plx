use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};

use crate::{
    core::{
        core_error::CoreInitError,
        editor::opener::EditorOpener,
        file_utils::file_handler::current_folder,
        parser::from_dir::FromDir,
        work::{work::Work, work_handler::WorkHandler, work_type::WorkType},
    },
    models::{event::Event, exo::Exo, project::Project, ui_state::UiState},
    ui::ui::Ui,
};

pub(super) struct State {
    pub(super) ui_state: UiState,
    pub(super) last_skill_index: usize,
    pub(super) last_exo_index: usize,
    pub(super) last_exo: Arc<Exo>,
    pub(super) last_skill_list: Arc<Vec<String>>,
    pub(super) last_exo_list: Arc<Vec<Exo>>,
}
impl State {
    pub fn new(first_exo: Arc<Exo>) -> Self {
        Self {
            ui_state: UiState::Home,
            last_skill_index: 0,
            last_exo_index: 0,
            last_skill_list: Arc::new(Vec::new()),
            last_exo_list: Arc::new(Vec::new()),
            last_exo: first_exo,
        }
    }
}
pub struct App {
    pub(super) state: State,
    pub(super) project: Project,
    pub(super) work_handler: Arc<Mutex<WorkHandler>>,
    pub(super) event_tx: Sender<Event>,
    pub(super) event_rx: Receiver<Event>,
    pub(super) ui_state_tx: Sender<UiState>,
    pub(super) run: bool,
}

impl App {
    pub fn new() -> Result<Self, CoreInitError> {
        let current_folder = match current_folder() {
            Ok(folder) => folder,
            Err(_err) => return Err(CoreInitError::PlxProjNotFound), // TODO maybe be more specific
                                                                     // here by adding the error detail
        };
        // TODO these warnings should be accessible to the user
        let (project, _warnings) = match Project::from_dir(&current_folder) {
            Ok((project, warnings)) => (project, warnings),
            Err((err, _warnings)) => {
                // TODO handle these warnings even in case of failure
                return Err(CoreInitError::ProjFilesParsingError(format!("{:?}", err)));
            }
        };
        let (event_tx, event_rx) = mpsc::channel();
        let (ui_state_tx, ui_state_rx) = mpsc::channel();
        let work_handler = WorkHandler::new(event_tx.clone());

        let mut app = App {
            state: State::new(Arc::new(project.skills[0].exos[0].clone())),
            project,
            work_handler,
            event_tx,
            event_rx,
            ui_state_tx,
            run: true,
        };
        app.start_ui(ui_state_rx);
        Ok(app)
    }

    pub(super) fn set_ui_state(&mut self, new_state: UiState) {
        self.ui_state_tx.send(new_state.clone());
        self.state.ui_state = new_state;
    }
    pub(super) fn resume_last_exo(&mut self) {
        //TODO we need to handle project state so we know where to resume the project from
        if let Some(exo) = &self.project.resume() {
            App::open_editor(&self.work_handler, exo);
            //TODO
            // self.state.ui_state = UiState::Compiling {
            //     exo: Arc::new((*exo).clone()),
            // }
        } else {
            //TODO tell the user we couldn't resume last exo
        }
    }
    pub fn run_forever(mut self) {
        while self.run {
            if let Ok(event) = self.event_rx.recv() {
                match event {
                    Event::KeyPressed(key) => self.on_key_press(key),
                    Event::EditorOpened => todo!(),
                    Event::CouldNotOpenEditor => todo!(),
                    Event::ProcessCreationFailed => todo!(),
                    Event::ProcessOutputLine(_) => todo!(),
                    Event::OutputCheckPassed(_) => todo!(),
                    Event::OutputCheckFailed(_, _) => todo!(),
                    Event::FileSaved => todo!(),
                }
            }
        }
    }
    // fn start_work(wh: &Arc<Mutex<WorkHandler>>, work: Box<dyn Work + Send>) {
    fn start_work(wh: &Arc<Mutex<WorkHandler>>, work: Box<dyn Work + Send>) {
        if let Ok(mut wh) = wh.lock() {
            wh.spawn_worker(work);
        }
    }
    fn start_ui(&mut self, ui_state_rx: Receiver<UiState>) {
        let ui = Ui::new(ui_state_rx);
        let _ = self.ui_state_tx.send(UiState::Home);
        App::start_work(&self.work_handler, Box::new(ui));
    }
    fn stop_ui(&mut self) {
        if let Ok(mut work_handler) = self.work_handler.lock() {
            work_handler.stop_workers(WorkType::Ui);
        }
    }

    fn open_editor(wh: &Arc<Mutex<WorkHandler>>, exo: &Exo) {
        if let Some(file) = exo.get_main_file() {
            if let Some(opener) = EditorOpener::new_default_editor(file.to_path_buf()) {
                App::start_work(wh, Box::new(opener));
            }
        }
    }
}
