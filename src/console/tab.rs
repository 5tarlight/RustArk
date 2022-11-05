use std::cmp;

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
            content: String::new(),
        }
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn build(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        let lines: Vec<&str> = self.content.split("\n").collect();
        for line in lines.iter() {
            if line.chars().count() <= self.width as usize {
                result.push(String::from(*line));
            } else {
                let mut cur = String::from(*line);
                while !cur.is_empty() {
                    let (chunk, rest) = cur.split_at(cmp::min(self.width as usize, cur.len()));
                    result.push(String::from(chunk));
                    cur = String::from(rest);
                }
            }
        }

        result
    }
}
