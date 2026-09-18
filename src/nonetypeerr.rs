use crate::process_user_mod::*;
use std::collections::HashMap;
pub fn find_user(user_id: i64) -> Option<User>{
    let users = HashMap::from([
        (1, User {id:1, name: "Alice".into() }),
        (2, User {id:2, name: "Bob".into() }),
    ]);
    users.get(&user_id).cloned()
}