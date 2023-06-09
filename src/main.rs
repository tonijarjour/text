mod document;
mod editor;
mod terminal;
use editor::Editor;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut editor = if args.len() > 1 {
        let file_path = &args[1];
        Editor::from(file_path.as_str())
    } else {
        Editor::default()
    };

    editor.run();
}
