use color_eyre::eyre::Result;

mod allocation {
    use chrono::{prelude::*, Duration};
    use std::ops::Add;

    #[derive(PartialEq, Debug)]
    pub struct Project {
        approx_end_date: Date<Utc>,
        approx_start_date: Date<Utc>,
        name: String,
        approx_value: u32,
    }

    impl Project {
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

        pub fn duration(self) -> Duration {
            self.approx_end_date - self.approx_start_date
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn has_duration() {
            let name = String::from("Project");
            let start = chrono::Utc.ymd(2014, 7, 8);
            let duration = chrono::Duration::weeks(2);
            let approx_value: u32 = 20000;
            let p = Project::new(name, approx_value, start, &duration);
            assert_eq!(p.duration(), duration)
        }
        #[test]
        fn new_function() {
            let name = String::from("Project");
            let start = chrono::Utc.ymd(2014, 7, 8);
            let duration = chrono::Duration::weeks(2);
            let end = start.add(duration);
            let approx_value: u32 = 20000;
            let p = Project::new(name, approx_value, start, &duration);
            assert_eq!(
                p,
                Project {
                    name: "Project".into(),
                    approx_value,
                    approx_start_date: start,
                    approx_end_date: end
                },
            )
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    Ok(())
}
