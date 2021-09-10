//! Authentication related types and functions.

mod apps;
pub use self::apps::create_authenticate_as_app_jwt;

pub enum Auth {
    None,
    PersonalToken(String),
}

impl Default for Auth {
    fn default() -> Self {
        Self::None
    }
}
