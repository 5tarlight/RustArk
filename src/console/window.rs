use std::vec;

use super::{clear, estimate_size, pause, quiet_pause, read_ch, tab::Tab};

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

    pub fn refresh(&mut self) {
        let size = estimate_size();
        self.width = size.0;
        self.height = size.1;
    }

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
    pub fn show(&mut self) {
        let mut index = 0;
        self.refresh();

        loop {
            clear();

            let headline = self.get_headline(index);
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

            read_ch();
        }
    }

    pub fn add_tab(&mut self, name: String) {
        self.tabs.push(Tab::new(name, self));
    }
}
