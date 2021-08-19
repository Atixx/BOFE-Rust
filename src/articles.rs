use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Article {
    pub title: String,
    pub link: String,
}

impl fmt::Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Title: {}, URL: {}", self.title, self.link)
    }
}
