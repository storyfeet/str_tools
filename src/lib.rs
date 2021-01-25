pub mod char_match;
pub mod str_chars;
pub mod string_util;

pub mod traits {
    pub use crate::char_match::CharMatch;
    pub use crate::str_chars::StrChars;
    pub use crate::string_util::StringUtil;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
