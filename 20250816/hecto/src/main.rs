use crate::edit::editor::Editor;

mod edit;
mod buf;
mod constant;
mod component;

fn main() {
    let mut editor = Editor::new().unwrap_or_default();
    editor.run();
}
