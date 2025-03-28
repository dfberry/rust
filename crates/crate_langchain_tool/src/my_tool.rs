use std::error::Error;
use serde_json::{json, Value};
use async_trait::async_trait;
use langchain_rust::tools::Tool;

pub use crate::my_feature::MyAddFeature;

#[async_trait]
impl Tool for MyAddFeature {
    async fn run(&self, input: Value) -> Result<String, Box<dyn Error + 'static>> {
        let a = input["a"].as_f64().ok_or("Missing or invalid 'a'".to_string())?;
        let b = input["b"].as_f64().ok_or("Missing or invalid 'b'".to_string())?;
        let sum = a + b;
        Ok(format!("The sum of {} and {} is {}", a, b, sum))
    }

    fn name(&self) -> String {
        "MyTool".to_string()
    }

    fn description(&self) -> String {
        "A tool that adds two numbers together.".to_string()
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "a": { "type": "number" },
                "b": { "type": "number" }
            },
            "required": ["a", "b"]
        })
    }
}

impl Default for MyAddFeature {
    fn default() -> MyAddFeature {
        MyAddFeature {}
    }
}

#[cfg(test)]
mod tests {
    use super::MyAddFeature;
    use serde_json::json;
    use langchain_rust::tools::Tool;

    #[tokio::test]
    async fn test_adder_tool() {
        let tool = MyAddFeature::default();
        let input = json!({ "a": 3.0, "b": 5.0 });

        let result = tool.run(input).await.unwrap();
        assert_eq!(result, "The sum of 3 and 5 is 8");
    }

    #[test]
    fn test_add_function() {
        assert_eq!(MyAddFeature::add(2.0, 3.0).unwrap(), 5.0);
        assert_eq!(MyAddFeature::add(-1.0, 1.0).unwrap(), 0.0);
    }
}