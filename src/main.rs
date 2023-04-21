mod editor;
mod terminal;
use crate::editor::Editor;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
