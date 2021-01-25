pub trait StringUtil {
    fn del_char(&mut self) -> Option<char>;
}

impl StringUtil for String {
    fn del_char(&mut self) -> Option<char> {
        let l = self.len();
        for x in 1..6 {
            if l < x {
                return None;
            }
            if let Some(_) = self.as_str().get(l - x..) {
                let c = self[(l - x)..].chars().next();
                self.remove(l - x);
                return c;
            }
        }
        None
    }
}
