

pub mod tui;
pub mod typer;


fn main() {

    //typer(text);
    let _ = tui::tui::gui("disabled".to_string(), false);
}


