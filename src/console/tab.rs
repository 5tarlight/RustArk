use super::window::Window;

pub struct Tab {
    width: u16,
    height: u16,
    pub content: String,
    pub name: String,
}

impl Tab {
    pub fn new(name: String, parent: &Window) -> Self {
        Self {
            width: parent.width - 2,
            height: parent.height - 2,
            name,
            content: String::from(""),
        }
    }
}
