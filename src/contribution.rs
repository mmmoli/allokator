use chrono::{Date, Utc};

pub trait Contribution {
    fn get_contribution_on(self, date: Date<Utc>) -> i32;
}
