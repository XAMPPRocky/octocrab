use crate::Octocrab;

pub struct ClassroomHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> ClassroomHandler<'octo> {
    pub fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }
}
