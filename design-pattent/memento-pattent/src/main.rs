pub mod editor;
pub mod editorstate;
pub mod history;

fn main() {
    let mut editor = editor::Editor::new();
    let mut history = history::History::new();

    editor.set_content(String::from("a"));
    history.push(editor.create_state());

    editor.set_content(String::from("b"));
    history.push(editor.create_state());

    editor.set_content(String::from("c"));
    editor.restore(history.pop().unwrap());

    println!("{}", editor.get_content());
}
