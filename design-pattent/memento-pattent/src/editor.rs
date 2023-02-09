use crate::editorstate::EditorState;
pub(crate) struct Editor {
    content: String,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }
    pub fn create_state(&self) -> EditorState {
        EditorState::new(self.content.clone())
    }
    pub fn restore(&mut self, state: EditorState) {
        self.content = state.get_content().clone();
    }
    pub fn get_content(&self) -> &String {
        &self.content
    }
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
}
