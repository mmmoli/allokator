use crate::contribution::Contribution;
use chrono::prelude::*;

/// # Resource
/// Represents a person or a thing attached to the project.
#[derive(PartialEq, Debug)]
pub struct Resource {
    pub end_date: Date<Utc>,
    pub start_date: Date<Utc>,
    pub name: String,
    approx_cost: u32,
}

impl Contribution for Resource {
    /// Returns the contribution for a given month.
    ///
    /// ### Example
    /// ```
    /// use chrono::{prelude::*, Duration};
    /// use allokator::projects::Project;    
    /// use allokator::contribution::Contribution;
    ///
    /// let name = String::from("My Project");
    /// let start = Utc.ymd(2014, 7, 8);
    /// let duration = Duration::weeks(2);
    /// let approx_value: u32 = 20000;
    /// let p = Project::new(name, approx_value, start, &duration);
    /// p.get_contribution_on(Utc.ymd(2014, 7, 8));
    /// ```    
    fn get_contribution_on(self, date: Date<Utc>) -> i32 {
        let before_start = self.start_date >= date;
        let after_end = self.end_date > date;
        match before_start || after_end {
            true => 0,
            false => 0,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn something() {
        assert!(true)
    }
}
