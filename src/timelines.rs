use crate::metrics::Revenue;
use chrono::{prelude::*, Duration};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, PartialEq, Eq)]
pub struct Timeline {
    pub name: String,
    pub start_date: NaiveDate,
    pub duration: Duration,
    pub metric: Revenue,
}

impl Default for Timeline {
    fn default() -> Self {
        Self {
            name: String::from("New Timeline"),
            start_date: NaiveDate::default(),
            duration: Duration::weeks(4),
            metric: Revenue { value: 10000 },
        }
    }
}

impl Timeline {
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a [`TimelineBuilder`] for a [`Timeline`].
    pub fn builder(name: &str) -> TimelineBuilder {
        TimelineBuilder::new(name)
    }

    /// Returns the end date of this [`Timeline`].
    #[allow(dead_code)]
    pub fn end_date(&self) -> NaiveDate {
        std::ops::Add::add(self.start_date, self.duration)
    }

    /// Returns the value of this metric on a given date.
    pub fn contribution_on(&self, date: NaiveDate) -> u32 {
        let in_scope = date >= self.start_date && date <= self.end_date();
        match in_scope {
            true => self.metric.value,
            false => 0,
        }
    }

    /// Returns a random variable within error bounds of this metric on a given date.
    pub fn roll(&self) -> (u32, NaiveDate) {
        let mut rng = rand::thread_rng();
        let half_date_variance = Duration::weeks(4);
        let min_date = self.start_date - half_date_variance;
        let max_duration = self.duration + half_date_variance + half_date_variance;
        let random_date =
            min_date + Duration::days(rand::random::<i64>() % max_duration.num_days());

        let random_value = {
            let price_options = [0.0, 0.8, 0.9, 1.0, 1.1];

            match price_options.choose(&mut rng) {
                Some(multipler) => (self.metric.value as f32 * multipler) as u32,
                None => self.metric.value,
            }
        };

        (random_value, random_date)
    }
}

pub struct TimelineBuilder {
    name: String,
    start_date: NaiveDate,
    duration: Duration,
    metric: Revenue,
}

impl TimelineBuilder {
    pub fn new(name: &str) -> TimelineBuilder {
        TimelineBuilder {
            name: name.to_owned(),
            start_date: NaiveDate::default(),
            duration: Duration::weeks(4),
            metric: Revenue { value: 10000 },
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
        TimelineBuilder::new("New Timeline")
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
            name: "My Timeline".to_string(),
            start_date: NaiveDate::default(),
            duration: Duration::weeks(4),
            metric: Revenue { value: 10000 },
        };
        let t_from_builder = TimelineBuilder::new("My Timeline").build();
        assert_eq!(t, t_from_builder);
    }

    #[test]
    fn test_contribution_on() {
        let t = Timeline {
            name: "New Timeline".to_string(),
            start_date: NaiveDate::from_ymd(2022, 1, 2),
            duration: Duration::weeks(4),
            metric: Revenue { value: 10000 },
        };
        assert_eq!(t.contribution_on(NaiveDate::from_ymd(2022, 1, 1)), 0);
        assert_eq!(t.contribution_on(NaiveDate::from_ymd(2022, 1, 2)), 10000);
        assert_eq!(t.contribution_on(NaiveDate::from_ymd(2022, 1, 3)), 10000);
        assert_eq!(t.contribution_on(NaiveDate::from_ymd(2022, 1, 30)), 10000);
        assert_eq!(t.contribution_on(NaiveDate::from_ymd(2022, 1, 31)), 0);
        assert_eq!(t.contribution_on(NaiveDate::from_ymd(2022, 2, 1)), 0);
    }
}
