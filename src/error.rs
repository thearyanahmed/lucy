pub enum LucyError {
    UrlNotFoundError,
    NotAValidUrlError
}

impl LucyError {
    pub fn to_string(&self) -> String {
        match *self {
            Self::UrlNotFoundError => "url not found".to_string(),
            Self::NotAValidUrlError => "not a valid url".to_string(),
        }
    }
}
