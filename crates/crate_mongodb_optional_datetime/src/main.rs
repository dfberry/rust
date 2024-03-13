  use bson::DateTime as BsonDateTime;

// pub fn string_to_optional_datetime(s: &String) -> Option<BsonDateTime> {
//   let bson_date_time: BsonDateTime = BsonDateTime::parse_rfc3339_str(s).unwrap();
//   let optional_date_time:Option<BsonDateTime> = bson_date_time.into();
//   return optional_date_time
// }
pub fn string_to_optional_datetime2(s: &String) -> Option<BsonDateTime> {
  match BsonDateTime::parse_rfc3339_str(s) {
    Ok(dt) => Some(dt.into()),
    Err(_) => None, // handle error as you see fit
  }
}



pub fn stringto_datetime(s: &String) -> BsonDateTime {
  let new_object = BsonDateTime::parse_rfc3339_str(s).unwrap();
  return new_object
}

#[tokio::main]
async fn main() {
    let existing_date: String = "2025-08-01T00:00:00Z".to_string();
    let empty_data: String = "".to_string();

    let date_time1 = string_to_optional_datetime2(&existing_date);
    let date_time2 = string_to_optional_datetime2(&empty_data);

    println!("optional_date_time: {:?}", date_time1);
    println!("optional_date_time: {:?}", date_time2);
}
