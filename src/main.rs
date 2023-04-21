mod editor;
mod terminal;
use editor::Editor;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
