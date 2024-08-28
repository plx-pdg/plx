pub mod core;
pub mod models;
pub mod ui;
use core::core::PlxCore;
use ui::ui::Ui;
fn main() -> io::Result<()> {
    let core = PlxCore::new();
    let ui = Ui::new(core);
    ui.loop_forever();
    Ok(())
}
