use allokator::timelines::Timeline;
use chrono::{Duration, NaiveDate};
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let t1: Timeline = Timeline {
        name: "Project 1".to_string(),
        start_date: NaiveDate::from_ymd(2022, 1, 1),
        duration: Duration::weeks(6),
    };

    let t2: Timeline = Timeline {
        name: "Project 2".to_string(),
        start_date: NaiveDate::from_ymd(2022, 1, 1),
        duration: Duration::weeks(6),
    };

    let timelines = vec![t1, t2];

    for t in timelines {
        println!("{:?}", t);
    }

    Ok(())
}
