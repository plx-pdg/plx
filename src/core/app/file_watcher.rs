use log::error;

use super::app::App;

impl App {
    /// File saved event handler
    /// Called when one the current exo files gets saved
    pub(super) fn on_file_save(&mut self) {
        if let Some(ref mut cr) = self.current_run {
            let compile = App::compile(&self.work_handler, &cr.exo);
            if let Err(err) = compile {
                error!("Error Starting Compilation {}", err);
            }
        }
    }
}
