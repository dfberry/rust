use std::error::Error;
use serde_json::{json, Value};
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;
use langchain_rust::tools::Tool;

use crate::my_feature::MyAddFeature;

struct MyToolResults {
    status: bool,
    error: Option<String>,
    value: Option<f64>
}

#[derive(Serialize, Deserialize)]
struct MyToolInput {
    a: f64,
    b: f64
}

#[async_trait]
impl Tool for MyAddFeature {
    fn name(&self) -> String {
        String::from("Tool_MyAddFeature")
    }

    fn description(&self) -> String {
        String::from(
            r#""Wrapper for MyAddFeature. "#
        )
    }

    fn run(&self, input: MyToolInput) -> Result<MyToolResults, Box<dyn Error>> {

        match MyAddFeature::add(input.a, input.b) {
            Ok(sum) => {
                let result = MyToolResults {
                    status: true,
                    error: None,
                    value: Some(sum)
                };
                println!("Tool_MyAddFeature: Successfully added numbers: {} + {} = {}", input.a, input.b, sum);
                Ok(result)
            }
            Err(e) => {
                println!("Tool_MyAddFeature: Error occurred: {}", e);
                Err(Box::new(e))
            }
        }
    }

    fn parameters(&self) -> Value {
        json!({
            "description": "Adds two numbers together.",
            "type": "object",
            "properties": {
                "a": {
                    "type": "number",
                    "description": "The first number"
                },
                "b": {
                    "type": "number",
                    "description": "The second number"
                }
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