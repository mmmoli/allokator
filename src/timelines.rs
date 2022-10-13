use chrono::{prelude::*, Duration};

#[derive(PartialEq, Debug)]
pub struct Timeline {
    pub name: String,
    pub start_date: NaiveDate,
    pub duration: Duration,
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
        };
        assert_eq!(t1.name, "test");
    }
}
