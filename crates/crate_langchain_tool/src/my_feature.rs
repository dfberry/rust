use std::fmt::Error;

pub struct MyAddFeature {
    }

impl MyAddFeature {
    pub fn add(a: f64, b: f64) -> Result<f64, Error> {
        Ok(a + b)
    }
}

#[cfg(test)]
mod tests {
    use super::MyAddFeature;

    #[test]
    fn test_add_function() {
        assert_eq!(MyAddFeature::add(2.0, 3.0).unwrap(), 5.0);
        assert_eq!(MyAddFeature::add(-1.0, 1.0).unwrap(), 0.0);
    }
}