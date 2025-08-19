use crate::editor::editor::Editor;

mod editor;
mod buf;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
