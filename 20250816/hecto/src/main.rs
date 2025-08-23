use crate::edit::editor::Editor;

mod edit;
mod buf;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
