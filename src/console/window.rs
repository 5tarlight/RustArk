use core::panic;
use std::{
    cmp::{self, max_by},
    vec,
};

use super::{clear, estimate_size, read_ch, tab::Tab};

/// Console CUI manager.
/// calculate console's width and height and handle with tabs.
pub struct Window {
    pub width: u16,
    pub height: u16,
    pub tabs: Vec<Tab>,
}

impl Window {
    /// Create new window object with predefined width and height.
    pub fn new(w: u16, h: u16) -> Self {
        Self {
            width: w,
            height: h,
            tabs: vec![],
        }
    }

    /// Automatically refresh width and height of console.
    /// This will not refresh cui but, internal varaibles.
    pub fn refresh(&mut self) {
        let size = estimate_size();
        self.width = size.0;
        self.height = size.1;
    }

    /// Build headline which lists titles of resistered tabs.
    /// Length of content is fitted to console's width
    /// Not sufficient width may cause visual problem.
    fn get_headline(&self, index: usize) -> String {
        let names: Vec<String> = self
            .tabs
            .iter()
            .map(|me| {
                if me.name.eq(&self.tabs[index].name) {
                    format!("✔{}", me.name)
                } else {
                    String::from(&me.name)
                }
            })
            .collect();

        format!("┤ {} ├", names.join(" | "))
    }

    // tab: 9
    // 1: 49
    // h: 104
    // j: 106
    // k : 107
    // l : 108
    // q : 113
    /// Print content of window fit to console.
    /// Switch tab with "tab" key and move with "hjkl".
    /// Reading arrow keys is not supported.
    pub fn show(&mut self) {
        let mut index: usize = 0;
        let mut scroll: usize = 0;
        let mut input: u8 = 0;
        let content_height = self.height - 2;
        self.refresh();

        let mut content: Vec<String> = Vec::new();

        loop {
            // clear();
            // println!("{}", input);
            if input == 9 {
                if index < self.tabs.len() - 1 {
                    index = index + 1;
                } else {
                    index = 0;
                }
                scroll = 0;
            } else if input == 113 {
                break;
            } else if input == 106 {
                if (scroll as isize) < (content.len() as isize) - (self.height as isize) + 3 {
                    scroll = scroll + 1;
                }
            } else if input == 107 {
                if scroll > 0 {
                    scroll = scroll - 1;
                }
            }

            content = self.tabs[index].build();

            let div = cmp::max(content.len() as isize - self.height as isize, 0) as usize + 3;
            let percent = scroll as f64 / div as f64;
            let position = (content_height as f64 * percent) as usize + 1;
            let position = cmp::min(position, content_height as usize);

            let headline = self.get_headline(index);
            let headline_len = headline.chars().count();
            let required_w = max_by(headline_len, 2 as usize, |x: &usize, y: &usize| x.cmp(y));
            if required_w > self.width as usize {
                panic!("Width is too short!")
            }

            let required_h = 2;
            if required_h > self.height as usize {
                panic!("Height is too short!")
            }

            for i in 0..self.height {
                for j in 0..self.width {
                    if i == 0 {
                        // first line
                        if j == 0 {
                            print!("┌")
                        } else if j == self.width - 1 {
                            print!("┐")
                        } else if (j as usize) <= headline_len {
                            // title
                            print!("{}", headline.chars().nth((j - 1) as usize).unwrap());
                        } else {
                            print!("─")
                        }
                    } else if i == self.height - 1 {
                        // last line
                        if j == 0 {
                            print!("└")
                        } else if j == self.width - 1 {
                            print!("┘")
                        } else {
                            print!("─")
                        }
                    } else {
                        // rest line
                        if j == 0 || j == self.width - 1 {
                            // ┃ │
                            if i as usize == position && j != 0 {
                                print!("❚")
                            } else {
                                print!("│")
                            }
                        } else {
                            if let Some(line) = content.get((i - 1 + scroll as u16) as usize) {
                                if let Some(ch) = line.chars().nth((j - 1) as usize) {
                                    print!("{}", ch);
                                } else {
                                    print!(" ");
                                }
                            } else {
                                print!(" ");
                            }
                        }
                    }
                }

                if i != self.height - 1 {
                    println!("");
                }
            }

            input = read_ch();
        }
    }

    /// Add new empty tab with name.
    /// This will not automatically re-draw CUI.
    pub fn add_tab(&mut self, name: String) {
        self.tabs.push(Tab::new(name, self));
    }
}
