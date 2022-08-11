use chrono::{prelude::*, Duration};
use std::ops::Add;

#[derive(PartialEq, Debug)]
pub enum ProjectBuilderError {
    ZeroLengthDuration,
}

impl std::error::Error for ProjectBuilderError {}
impl std::fmt::Display for ProjectBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ProjectBuilderError::ZeroLengthDuration => write!(f, "Project has no duration."),
        }
    }
}

/// # ProjectBuilder
/// Constructs Projects.
#[derive(PartialEq, Debug)]
pub struct ProjectBuilder {
    duration: Duration,
    name: String,
    start_date: Date<Utc>,
    value: u32,
}

impl ProjectBuilder {
    pub fn new() -> ProjectBuilder {
        // Set the minimally required fields of Foo.
        ProjectBuilder {
            duration: Duration::weeks(4),
            name: "New Project".into(),
            start_date: Utc::today(),
            value: 20000,
        }
    }

    /// This method sets the project's name.
    ///
    /// ## Example
    /// ```
    /// use allokator::projects::ProjectBuilder;
    /// let project = ProjectBuilder::new()
    ///   .name("My New Project".into())
    ///   .build();
    /// assert_eq!(project.name, "My New Project".to_string())
    /// ```
    pub fn name(mut self, name: String) -> ProjectBuilder {
        self.name = name;
        self
    }

    /// This method sets the project's duration.
    ///
    /// ## Example
    /// ```
    /// use allokator::projects::ProjectBuilder;
    /// use chrono::Duration;
    ///
    /// let duration = Duration::weeks(8);
    /// let project = ProjectBuilder::new()
    ///   .duration(duration)
    ///   .build();
    /// assert_eq!(project.duration(), duration)
    /// ```
    pub fn duration(mut self, duration: Duration) -> ProjectBuilder {
        self.duration = duration;
        self
    }

    /// This method sets the project's start date.
    ///
    /// ## Example
    /// ```
    /// use allokator::projects::ProjectBuilder;
    /// use chrono::prelude::*;
    ///
    /// let start = Utc.ymd(2014, 7, 8);
    /// let project = ProjectBuilder::new()
    ///   .start_date(start)
    ///   .build();
    /// assert_eq!(project.start_date, start)
    /// ```
    pub fn start_date(mut self, date: Date<Utc>) -> ProjectBuilder {
        self.start_date = date;
        self
    }

    pub fn build(self) -> Project {
        Project {
            end_date: self.start_date + self.duration,
            start_date: self.start_date,
            approx_value: self.value,
            name: self.name,
        }
    }
}

/// # Project
/// Represents a piece of work we might do in the future.
/// Note: all values are designed to be approximate.
#[derive(PartialEq, Debug)]
pub struct Project {
    pub end_date: Date<Utc>,
    pub start_date: Date<Utc>,
    pub name: String,
    approx_value: u32,
}

/// Returns
/// Note: all values are designed to be approximate.
impl Default for Project {
    fn default() -> Self {
        let start = Utc::today() + Duration::weeks(3);
        let end = start + Duration::weeks(4);

        Project {
            end_date: end,
            start_date: start.into(),
            approx_value: 20000,
            name: "New Project".into(),
        }
    }
}

impl Project {
    /// Creates a new Project with an approx value, start date and duration.
    ///
    /// ## Example
    /// ```
    /// use chrono::{prelude::*, Duration};
    /// use allokator::projects::Project;
    ///
    /// let name = String::from("My Project");
    /// let start = Utc.ymd(2014, 7, 8);
    /// let duration = Duration::weeks(2);
    /// let approx_value: u32 = 20000;
    /// let p = Project::new(name, approx_value, start, &duration);
    /// ```
    pub fn new(
        name: String,
        approx_value: u32,
        approx_start_date: Date<Utc>,
        duration: &chrono::Duration,
    ) -> Project {
        let approx_end_date = approx_start_date.add(*duration);
        Project {
            end_date: approx_end_date,
            start_date: approx_start_date,
            approx_value,
            name,
        }
    }

    /// Returns the Project's approximate duration.
    ///
    /// ## Example
    /// ```
    /// use chrono::{prelude::*, Duration};
    /// use allokator::projects::Project;
    ///
    /// let name = String::from("My Project");
    /// let start = Utc.ymd(2014, 7, 8);
    /// let duration = Duration::weeks(2);
    /// let approx_value: u32 = 20000;
    /// let p = Project::new(name, approx_value, start, &duration);
    /// assert_eq!(p.duration(), duration)
    /// ```
    pub fn duration(self) -> Duration {
        self.end_date - self.start_date
    }

    /// Returns the Project's approximate value.
    ///
    /// ## Example
    /// ```
    /// use chrono::{prelude::*, Duration};
    /// use allokator::projects::Project;
    ///
    /// let name = String::from("My Project");
    /// let start = Utc.ymd(2014, 7, 8);
    /// let duration = Duration::weeks(2);
    /// let approx_value: u32 = 20000;
    /// let p = Project::new(name, approx_value, start, &duration);
    /// assert_eq!(p.value(), approx_value)
    /// ```
    pub fn value(self) -> u32 {
        self.approx_value
    }

    /// Returns the contribution for a given month.
    ///
    /// ### Example
    /// ```
    /// use chrono::{prelude::*, Duration};
    /// use allokator::projects::Project;    
    ///
    /// let name = String::from("My Project");
    /// let start = Utc.ymd(2014, 7, 8);
    /// let duration = Duration::weeks(2);
    /// let approx_value: u32 = 20000;
    /// let p = Project::new(name, approx_value, start, &duration);
    /// p.get_contribution_at(Utc.ymd(2014, 7, 8));
    /// ```    
    pub fn get_contribution_at(self, date: Date<Utc>) -> i32 {
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
    fn random_contribution() {
        let name = String::from("My Project");
        let start = Utc.ymd(2022, 7, 8);
        let duration = Duration::weeks(2);
        let approx_value: u32 = 20000;
        let p = Project::new(name, approx_value, start, &duration);
        let during = start + Duration::weeks(1);
        let contribution = p.get_contribution_at(during);
        assert!(contribution >= 0);
        assert!(contribution <= approx_value as i32)
    }

    #[test]
    fn contribution_in_past() {
        let name = String::from("My Project");
        let start = Utc.ymd(2022, 7, 8);
        let duration = Duration::weeks(2);
        let approx_value: u32 = 20000;
        let p = Project::new(name, approx_value, start, &duration);
        let contribution = p.get_contribution_at(Utc.ymd(2014, 7, 8));
        assert_eq!(contribution, 0)
    }

    #[test]
    fn contribution_in_future() {
        let name = String::from("My Project");
        let start = Utc.ymd(2022, 7, 8);
        let duration = Duration::weeks(2);
        let approx_value: u32 = 20000;
        let p = Project::new(name, approx_value, start, &duration);
        let contribution = p.get_contribution_at(Utc.ymd(2024, 7, 8));
        assert_eq!(contribution, 0)
    }
}
