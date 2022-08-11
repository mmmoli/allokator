use chrono::{prelude::*, Duration};
use std::ops::Add;

/// # Project
/// Represents a piece of work we might do in the future.
/// Note: all values are designed to be approximate.
#[derive(PartialEq, Debug)]
pub struct Project {
    approx_end_date: Date<Utc>,
    approx_start_date: Date<Utc>,
    name: String,
    approx_value: u32,
}

impl Project {
    /// Creates a new Project with an approx value, start date and duration.
    ///
    /// ## Example
    /// ```
    /// use chrono::{prelude::*, Duration};
    /// use allokator::projects::Project;
    /// use std::ops::Add;
    ///
    /// let name = String::from("My Project");
    /// let start = chrono::Utc.ymd(2014, 7, 8);
    /// let duration = chrono::Duration::weeks(2);
    /// let end = start.add(duration);
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
            approx_end_date,
            approx_start_date,
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
    /// use std::ops::Add;
    ///
    /// let name = String::from("My Project");
    /// let start = chrono::Utc.ymd(2014, 7, 8);
    /// let duration = chrono::Duration::weeks(2);
    /// let end = start.add(duration);
    /// let approx_value: u32 = 20000;
    /// let p = Project::new(name, approx_value, start, &duration);
    /// assert_eq!(p.approx_duration(), duration)
    /// ```
    pub fn approx_duration(self) -> Duration {
        self.approx_end_date - self.approx_start_date
    }

    /// Returns the Project's approximate value.
    ///
    /// ## Example
    /// ```
    /// use chrono::{prelude::*, Duration};
    /// use allokator::projects::Project;
    /// use std::ops::Add;
    ///
    /// let name = String::from("My Project");
    /// let start = chrono::Utc.ymd(2014, 7, 8);
    /// let duration = chrono::Duration::weeks(2);
    /// let end = start.add(duration);
    /// let approx_value: u32 = 20000;
    /// let p = Project::new(name, approx_value, start, &duration);
    /// assert_eq!(p.approx_value(), approx_value)
    /// ```
    pub fn approx_value(self) -> u32 {
        self.approx_value
    }
}
