use std::sync::Arc;

pub fn convert_vec_to_json<T>(data: Vec<T>) -> Vec<Arc<serde_json::Value>>
where
    T: serde::Serialize,
{
    println!("convert_vec_to_json");
    data.into_iter().map(convert_to_json).collect()
}

pub fn convert_to_json<T>(data: T) -> Arc<serde_json::Value>
where
    T: serde::Serialize,
{
    println!("convert_to_json");
    let json = serde_json::to_value(data).unwrap();
    Arc::new(json)
}