/// GitHub Classroom API
mod assignments;
#[allow(clippy::module_inception)]
mod classroom;

pub use self::{assignments::*, classroom::*};
