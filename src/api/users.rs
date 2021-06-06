use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub age: i32,
}

impl User {
    pub async fn new(name: String, age: i32) -> User {
        let user = User {
            name: name.to_string(),
            age,
        };

        user
    }

    pub async fn get_all() -> Vec<User> {
        return vec![User {
            name: "Jonathan Rwabahizi".to_string(),
            age: 25,
        }];
    }
}
