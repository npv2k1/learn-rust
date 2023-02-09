pub(crate) struct EditorState {
    content: String,
}
impl EditorState {
    pub fn new(content: String) -> Self {
        Self { content }
    }
    pub fn get_content(&self) -> &String {
        &self.content
    }
}
