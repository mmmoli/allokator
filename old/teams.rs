use crate::{allocations::Allocation, traits::Contribution};
use chrono::prelude::*;

/// # Resource
/// Represents a person or a thing attached to the project.
#[derive(PartialEq, Debug)]
pub struct TeamMember {
    pub allocation: Allocation,
    pub name: String,
    approx_cost: u32,
}

impl Contribution for TeamMember {
    fn get_contribution_on(&self, date: &Date<Utc>) -> u32 {
        match self.allocation.is_active_on(date) {
            true => self.approx_cost,
            false => 0 as u32,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn something() {
        assert!(true)
    }
}
