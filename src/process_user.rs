#[warn(unused)]
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
#[warn(dead_code)]
#[derive(Clone)]
pub struct User{
    pub id: i64,
    pub name: String,
}

pub fn process_user(user_id: i64, name: &str) -> User{
    User {
        id: user_id,
        name: name.to_uppercase(),
    }
}


// let input: UserInput = serde_json::from_str(json_str)?;
// process_user(input.id, &input.name);

// process_user("not-a-number", 42);        // ❌ Compile error: expected i64, found &str
// process_user(None, "Alice");             // ❌ Compile error: expected i64, found Option
// Extra arguments are always a compile error.

// Deserializing JSON is type-safe too;