use crate::editor::Editor;

mod editor;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
