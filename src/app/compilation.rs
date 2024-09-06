use super::app::App;

/// Functions related to handling compilation events
impl App {
    /// Compilation output event handler
    /// Gets called when the compilation process outputs a new line
    pub(super) fn on_compilation_start(&mut self) {
        if let Some(ref mut cr) = self.current_run {
            cr.compilation_output.clear();
        }
        self.go_to_compiling();
    }
    pub(super) fn on_compilation_output(&mut self, line: String) {
        if let Some(ref mut cr) = self.current_run {
            cr.compilation_output.push(line);
        }
    }

    /// Compilation finished event handler
    /// Gets called when the target binary compilation ends
    pub(super) fn on_compilation_end(&mut self, success: bool) {
        if success {
            self.start_runners();
            if let Some(ref cr) = self.current_run {
                self.go_to_check_results(
                    0,
                    cr.check_results
                        .iter()
                        .map(|result| result.state.clone())
                        .collect(),
                );
            }
        } else {
            let output = if let Some(ref cr) = self.current_run {
                cr.compilation_output.join("\n")
            } else {
                String::from("")
            };
            self.go_to_compilation_error(0, output)
        }
    }
}
