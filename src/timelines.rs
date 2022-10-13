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
    pub fn end_date(&self) -> NaiveDate {
        self.start_date.add(self.duration)
    }

    /// Returns the value of this metric on a given date.
    pub fn contribution_on(&self, date: NaiveDate) -> u32 {
        let in_scope = date >= self.start_date && date <= self.end_date();
        match in_scope {
            true => self.metric.value,
            false => 0,
        }
    }
}

pub struct TimelineBuilder {
    name: String,
    start_date: NaiveDate,
    duration: Duration,
    metric: Revenue,
}

impl TimelineBuilder {
    pub fn new() -> TimelineBuilder {
        TimelineBuilder {
            name: "New Timeline".to_owned(),
            start_date: NaiveDate::default(),
            duration: Duration::weeks(4),
            metric: Revenue { value: 2000 },
        }
    }

    pub fn build(self) -> Timeline {
        Timeline {
            name: self.name,
            start_date: self.start_date,
            duration: self.duration,
            metric: self.metric,
        }
    }

    pub fn name(mut self, name: &str) -> TimelineBuilder {
        self.name = name.into();
        self
    }

    pub fn revenue(mut self, revenue: u32) -> TimelineBuilder {
        self.metric = Revenue { value: revenue };
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

    #[test]
    fn test_contribution_on() {
        let t = Timeline {
            name: "New Timeline".to_string(),
            start_date: NaiveDate::from_ymd(2022, 1, 2),
            duration: Duration::weeks(4),
            metric: Revenue { value: 2000 },
        };
        assert_eq!(t.contribution_on(NaiveDate::from_ymd(2022, 1, 1)), 0);
        assert_eq!(t.contribution_on(NaiveDate::from_ymd(2022, 1, 2)), 2000);
        assert_eq!(t.contribution_on(NaiveDate::from_ymd(2022, 1, 3)), 2000);
        assert_eq!(t.contribution_on(NaiveDate::from_ymd(2022, 1, 30)), 2000);
        assert_eq!(t.contribution_on(NaiveDate::from_ymd(2022, 1, 31)), 0);
        assert_eq!(t.contribution_on(NaiveDate::from_ymd(2022, 2, 1)), 0);
    }
}
