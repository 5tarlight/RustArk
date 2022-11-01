use super::window::Window;

pub struct Tab {
    width: u16,
    height: u16,
    pub content: String,
    pub name: String,
}

impl Tab {
    pub fn new(&self, name: String, parent: &Window) -> Self {
        Self {
            width: parent.width - 2,
            height: parent.height - 2,
            name,
            content: String::from(""),
        }
    }

    pub fn register(&self, name: String, parent: &Window) -> Self {
        let me = self.new(name, parent);
        // Register to parent window
        me
    }
}
