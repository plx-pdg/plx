pub mod core;
pub mod models;
pub mod ui;
// fn main() {
//     println!("PLX - Practice programming exos in a delightful Learning eXperience");
//     println!("Testing CI");
//     println!("Testing CI second time");
// }
use core::core::PlxCore;
use ui::ui::Ui;
fn main() -> io::Result<()> {
    let core = PlxCore::new();
    let ui = Ui::new(core);
    ui.loop_forever();
    Ok(())
}
