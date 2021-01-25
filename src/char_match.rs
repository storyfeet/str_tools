pub trait CharMatch {
    fn char_match(&self, c: char) -> bool;
}

impl CharMatch for str {
    fn char_match(&self, c: char) -> bool {
        self.contains(c)
    }
}
impl CharMatch for &str {
    fn char_match(&self, c: char) -> bool {
        self.contains(c)
    }
}
impl CharMatch for char {
    fn char_match(&self, c: char) -> bool {
        *self == c
    }
}

impl<F: Fn(char) -> bool> CharMatch for F {
    fn char_match(&self, c: char) -> bool {
        self(c)
    }
}
