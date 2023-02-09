use crate::editorstate::EditorState;
pub(crate) struct History {
    states: Vec<EditorState>,
}
impl History {
    pub fn new() -> Self {
        Self { states: Vec::new() }
    }
    pub fn push(&mut self, state: EditorState) {
        self.states.push(state);
    }
    pub fn pop(&mut self) -> Option<EditorState> {
        self.states.pop()
    }
}
