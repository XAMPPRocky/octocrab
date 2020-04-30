pub enum Auth {
    None,
    PersonalToken(String),
}

impl Default for Auth {
    fn default() -> Self {
        Self::None
    }
}
