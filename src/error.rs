pub enum LucyError {
    UrlNotFoundError,
    NotAValidError
}

impl LucyError {
    pub fn to_string(&self) -> String {
        match *self {
            Self::UrlNotFoundError => "url not found".to_string(),
            Self::NotAValidError => "not a valid url".to_string(),
        }
    }
}
