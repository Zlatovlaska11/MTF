pub mod tui;
pub mod typer;

fn main() {
    let _ = tui::tui::gui("disabled".to_string(), false);
}
