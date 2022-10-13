pub trait Metric {
    fn value(&self) -> u32;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Revenue {
    pub value: u32,
}

impl Metric for Revenue {
    fn value(&self) -> u32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_revenue() {
        let revenue = Revenue { value: 100 };
        assert_eq!(revenue.value, 100);
    }
}
