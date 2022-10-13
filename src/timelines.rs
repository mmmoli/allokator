use crate::metrics::Revenue;
use chrono::{prelude::*, Duration};
use std::ops::Add;

#[derive(Debug, PartialEq, Eq)]
pub struct Timeline {
    pub name: String,
    pub start_date: NaiveDate,
    pub duration: Duration,
    pub metric: Revenue,
}

impl Timeline {
    /// Creates a [`TimelineBuilder`] for a [`Timeline`].
    pub fn builder() -> TimelineBuilder {
        TimelineBuilder::new()
    }

    /// Returns the end date of this [`Timeline`].
    #[allow(dead_code)]
    fn end_date(&self) -> NaiveDate {
        self.start_date.add(self.duration)
    }
}

pub struct TimelineBuilder {
    name: String,
    start_date: NaiveDate,
    duration: Duration,
}

impl TimelineBuilder {
    pub fn new() -> TimelineBuilder {
        TimelineBuilder {
            name: "New Timeline".to_owned(),
            start_date: NaiveDate::default(),
            duration: Duration::weeks(4),
        }
    }

    pub fn build(self) -> Timeline {
        Timeline {
            name: self.name,
            start_date: self.start_date,
            duration: self.duration,
            metric: Revenue { value: 2000 },
        }
    }

    pub fn name(mut self, name: &str) -> TimelineBuilder {
        self.name = name.into();
        self
    }

    pub fn start_date(mut self, date: NaiveDate) -> TimelineBuilder {
        self.start_date = date;
        self
    }

    pub fn weeks(mut self, weeks: u8) -> TimelineBuilder {
        self.duration = Duration::weeks(weeks as i64);
        self
    }
}

impl Default for TimelineBuilder {
    fn default() -> TimelineBuilder {
        TimelineBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_timelines() {
        let t1 = Timeline {
            name: "test".to_string(),
            start_date: NaiveDate::from_ymd(2022, 1, 1),
            duration: Duration::weeks(6),
            metric: Revenue { value: 0 },
        };
        assert_eq!(t1.name, "test");
    }

    #[test]
    fn has_end_date() {
        let t = Timeline {
            name: "test".to_string(),
            start_date: NaiveDate::from_ymd(2022, 1, 1),
            duration: Duration::weeks(1),
            metric: Revenue { value: 0 },
        };
        assert_eq!(t.end_date(), NaiveDate::from_ymd(2022, 1, 8));
    }

    #[test]
    fn builder_test() {
        let t = Timeline {
            name: "New Timeline".to_string(),
            start_date: NaiveDate::default(),
            duration: Duration::weeks(4),
            metric: Revenue { value: 2000 },
        };
        let t_from_builder = TimelineBuilder::new().build();
        assert_eq!(t, t_from_builder);
    }
}
