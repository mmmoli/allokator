#[derive(PartialEq, Debug)]
pub struct Resource {
    name: String,
    contribution_value: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_resources() {
        let resource = Resource {
            name: "test".to_string(),
            contribution_value: 2000.0,
        };
        assert_eq!(resource.name, "test");
        assert_eq!(resource.contribution_value, 2000.0);
    }
}
