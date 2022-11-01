use super::{clear, quiet_pause};

pub struct Window {
    pub width: u16,
    pub height: u16,
}

impl Window {
    pub fn new(w: u16, h: u16) -> Self {
        Self {
            width: w,
            height: h,
        }
    }

    pub fn show(&self) {
        clear();

        for i in 0..self.height {
            for j in 0..self.width {
                if i == 0 {
                    // first line
                    if j == 0 {
                        print!("┌")
                    } else if j == self.width - 1 {
                        print!("┐")
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
}
