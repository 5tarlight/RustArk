use std::vec;

use super::{clear, quiet_pause, tab::Tab};

pub struct Window {
    pub width: u16,
    pub height: u16,
    tabs: Vec<Tab>,
}

impl Window {
    pub fn new(w: u16, h: u16) -> Self {
        Self {
            width: w,
            height: h,
            tabs: vec![],
        }
    }

    fn get_headline(&self) -> String {
        let names: Vec<String> = self.tabs.iter().map(|me| String::from(&me.name)).collect();

        format!("┤ {} ├", names.join(" | "))
    }

    pub fn show(&self) {
        clear();

        let headline = self.get_headline();
        let required = headline.chars().count();
        if required > self.width as usize {
            panic!("Width is too short!")
        }

        for i in 0..self.height {
            for j in 0..self.width {
                if i == 0 {
                    // first line
                    if j == 0 {
                        print!("┌")
                    } else if j == self.width - 1 {
                        print!("┐")
                    } else if (j as usize) <= required {
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
                        print!("│")
                    } else {
                        print!(" ")
                    }
                }
            }

            if i != self.height - 1 {
                println!("");
            }
        }

        quiet_pause();
    }

    pub fn add_tab(&mut self, name: String) {
        self.tabs.push(Tab::new(name, self));
    }
}
