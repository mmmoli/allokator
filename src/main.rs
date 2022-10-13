use allokator::timelines::Timeline;
use chrono::NaiveDate;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    let t1: Timeline = Timeline::builder("Project 1")
        .start_date(NaiveDate::from_ymd(2022, 1, 1))
        .weeks(4)
        .revenue(33333)
        .build();

    let t2: Timeline = Timeline::builder("Project 2")
        .start_date(NaiveDate::from_ymd(2022, 5, 20))
        .weeks(4)
        .revenue(3333)
        .build();

    let timelines = vec![t1, t2];

    loop {
        for t in &timelines {
            println!("{:?}", t.roll());
        }
    }
}
