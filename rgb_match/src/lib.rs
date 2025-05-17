#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let mut swap_first = String::new();
        let mut swap_second = String::new();
        if first == self.a {
            swap_first = "a".to_string();
        } else if first == self.b {
            swap_first = "b".to_string();
        } else if first == self.r {
            swap_first = "r".to_string();
        } else if first == self.g {
            swap_first = "g".to_string();
        }
        if second == self.a {
            swap_second = "a".to_string();
        } else if second == self.b {
            swap_second = "b".to_string();
        } else if second == self.r {
            swap_second = "r".to_string();
        } else if second == self.g {
            swap_second = "g".to_string();
        }

        if swap_first == "a" {
            self.a = second
        } else if swap_first == "b" {
            self.b = second
        } else if swap_first == "g" {
            self.g = second
        } else if swap_first == "r" {
            self.r = second
        }
        if swap_second == "a" {
            self.a = first
        } else if swap_second == "b" {
            self.b = first
        } else if swap_second == "g" {
            self.g = first
        } else if swap_second == "r" {
            self.r = first
        }
        self
    }
}
